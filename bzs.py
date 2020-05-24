# initial parsing from mrcheezes skyward-sword-tools
# parsing of stage and room files

from collections import OrderedDict
import struct

from utils import unpack, toStr

nodestruct='>4shhi'
nodestructnames="name count ff offset"

def parseBzs(data):
    name,count,ff,offset = struct.unpack('>4shhi',data[:12])
    assert ff == -1
    name = name.decode('ascii')
    return parseObj(name, count, data[offset:])

def parseObj(objtype, quantity, data):
    if objtype == 'V001':
        #root
        parsed = OrderedDict()
        for i in range(quantity):
            addr = i*12
            name,count,ff,offset = struct.unpack('>4shhi',data[addr:addr+12])
            assert ff == -1
            name = name.decode('ascii')
            parsed[name]=parseObj(name,count,data[addr+offset:])
            #if name != 'LAY ':
            #    parsed[name]=len(parsed[name])
        return parsed
    elif objtype == 'LAY ':
        #different layers of the room (always 29 of them)
        assert quantity == 29
        parsed = OrderedDict()
        for i in range(quantity):
            addr = i*8
            count,ff,offset = struct.unpack('>hhi',data[addr:addr+8])
            if count == 0:
                parsed['l%d'%i] = None
            else:
                parsed['l%d'%i] = parseObj('V001',count,data[addr+offset:])
        return parsed
            
    elif objtype in ('OBJN','ARCN'):
        parsed = []
        for i in range(quantity):
            addr = data[2*i]*0x100 + data[2*i+1]
            name = toStr(data[addr:])
            parsed.append(name)
        return parsed
    elif objtype == 'RMPL':
        parsed = OrderedDict()
        for i in range(quantity):
            rmpldata = data[4*i:]
            rmpl_id = rmpldata[0]
            count = rmpldata[1]
            addr = rmpldata[2]*0x100 + rmpldata[3]
            parsed[rmpl_id] = []
            for j in range(count):
                parsed[rmpl_id].append(rmpldata[addr+2*j:addr+2*j+2])
        return parsed

    else:
        # objects with quantities
        parsed = []
        structnames, structdef, size = objectstructs[objtype]
        for i in range(quantity):
            item = data[size*i:size*(i+1)]
            parsed.append(unpack(structnames, structdef, item))

        return parsed

objectstructs = {'FILE':('unk', '>4s', 4),
                'SCEN':('name room layer entrance byte4 byte5 flag6 zero flag8','>32sbbbbbbbb',40),
                'CAM ':('unk1 posx posy posz angle unk2 name','>4s3ff8s16s',44),
                'PATH':('unk', '>12s', 12),
                'PNT ':('unk', '>16s', 16),
                'SPNT':('unk', '>16s', 16),
                'BPNT':('pos1x pos1y pos1z pos2x pos2y pos2z pos3x pos3y pos3z unk','>3f3f3f4s',40),
                'SPTH':('unk', '>12s', 12),
                'AREA':('posx posy posz sizex sizey sizez angle area_link unk3 dummy','>3f3fHhb3s',32),
                'EVNT':('unk1 story_flag1 story_flag2 unk2 exit_id unk3 name','>2shh3sb14s32s',56),
                'PLY ':('byte1 byte2 play_cutscene byte4 posx posy posz unk2 entrance_id','>bbbb3f6sh',24),
                'OBJS':('unk1 unk2 posx posy posz                   unk3 angle unk4 unk5 name','>4s4s3fHHHH8s',36),
                'OBJ ':('unk1 unk2 posx posy posz                   unk3 angle unk4 unk5 name','>4s4s3fHHHH8s',36),
                'SOBS':('unk1 unk2 posx posy posz sizex sizey sizez unk3 angle unk4 unk5 name','>4s4s3f3fHHHH8s',48),
                'SOBJ':('unk1 unk2 posx posy posz sizex sizey sizez unk3 angle unk4 unk5 name','>4s4s3f3fHHHH8s',48),
                'STAS':('unk1 unk2 posx posy posz sizex sizey sizez unk3 angle unk4 unk5 name','>4s4s3f3fHHHH8s',48),
                'STAG':('unk1 unk2 posx posy posz sizex sizey sizez unk3 angle unk4 unk5 name','>4s4s3f3fHHHH8s',48),
                'SNDT':('unk1 unk2 posx posy posz sizex sizey sizez unk3 angle unk4 unk5 name','>4s4s3f3fHHHH8s',48),
                'DOOR':('unk1 unk2 posx posy posz                   unk3 angle unk4 unk5 name','>4s4s3fHHHH8s',36),
                'LYSE':('story_flag night layer','>hbb',4),
                'STIF':('wtf1 wtf2 wtf3 byte1 flagindex byte3 byte4 unk1 map_name_id unk2','>3fbbbb2sb1s',20),
                'PCAM':('pos1x pos1y pos1z pos2x pos2y pos2z angle wtf unk','>3f3fff4s',36),
                'LYLT':('layer demo_high demo_low dummy','>bbbb',4)}

def buildBzs(root: OrderedDict) -> bytes:
    count, odata = buildObj('V001', root)
    data=struct.pack(nodestruct, b'V001', count, -1, 12) + odata
    data+=(16-(len(data)%16))*b'\xff'
    return data

def buildObj(objtype, objdata) -> (int, bytes): # number of elements, bytes of body
    if objtype == 'V001':
        assert type(objdata) == OrderedDict
        offset=len(objdata)*12
        body=b''
        headerbytes=b''
        for typ, obj in objdata.items():
            count, data = buildObj(typ, obj)
            headerbytes+=struct.pack(nodestruct, typ.encode('ASCII'), count, -1, len(body)-len(headerbytes)+offset)
            body+=data
            # body+=(16-(len(body)%16))*b'\xFF'
        return (len(objdata), headerbytes+body)
    elif objtype == 'LAY ':
        assert type(objdata) == OrderedDict
        assert len(objdata) == 29
        offset=29*8
        body=b''
        headerbytes=b''
        for layer in objdata.values():
            if not layer:
                headerbytes+=struct.pack('>hhi', 0, -1, 0)
            else:
                count, data = buildObj('V001',layer)
                headerbytes+=struct.pack('>hhi', count, -1, len(body)-len(headerbytes)+offset)
                body+=data
        return (29, headerbytes+body)
            
    elif objtype in ('OBJN','ARCN'):
        assert type(objdata) == list
        offset=len(objdata)*2
        sbytes=b''
        headerbytes=b''
        for s in objdata:
            headerbytes+=struct.pack('>H', len(sbytes)+offset)
            sbytes+=s.encode('ASCII')+b'\x00'
        return (len(objdata), headerbytes+sbytes)
    elif objtype == 'RMPL':
        assert type(objdata) == OrderedDict
        offset=len(objdata)*4
        body=b''
        headerbytes=b''
        for i, s in objdata.items():
            headerbytes+=struct.pack('>BBH', i, len(s), len(body)+offset)
            body+=b'\x00'.join(s)
        return (len(objdata), headerbytes+body)

    else:
        assert type(objdata) == list
        _, structdef, _ = objectstructs[objtype]
        mapped = (struct.pack(structdef, *obj.values()) for obj in objdata)
        return (len(objdata), b''.join(mapped))