.open "main.dol"
; The subtype of TBox (chests) is determined by the item id
; change it, so that it uses 00 00 00 30 of params1 instead
.org 0x80269530 ; in AcOTBox::init
lwz r0, 0x4(r28) ; load params1
rlwinm r0,r0,28,30,31 ; r0 = (r0 >> 4) & 3
stb r0, 0x1209(r28) ; store subtype
b 0x80269554

.org 0x80115A04 ; in some function that is text advancing related
li r4, 1 ; enables instant text

; patch to not update sword model when getting an upgrade
.org 0x8005e2f0
stwu r1, -0x30(r1)
mflr r0
stw r0, 0x34(r1)
stw r31, 0x2C(r1)
mr r31, r3
stw r30, 0x28(r1)
bl updateCurrentSword
b 0x8005e378
.org 0x8005e384 ; branch to initModels, after actual model updates (which crash)
b 0x8005e250

; to make sure that all items randomized to freestanding locations
; force a textbox, make them subtype 9 and have them act like baby rattle
; TODO: this fix can probably replaced with this:
;.org 0x80256c30
;lwz r3, 0x4(r3) ; load params1
;rlwinm r3,r3,0xc,0x1c,0x1f ; extract subtype
;b 0x802476e0 ; branch to rest of function, that returns if it's 9
;.org 0x802476e0
;subi r0, r3, 9 ; subtype 9, normally checks for rattle itemid
.org 0x80256c30
lwz r3, 0x4(r3) ; load params1
b 0x8024a1c0 ; get subtype out of params1
.org 0x80254150
cmpwi r3, 9
beq 0x80254168

; don't show fi text after map
.org 0x80252b48
b 0x80252bb4

; function that checks if the item is the bird statuette
; always return false to fix the animation
.org 0x80250b00
li r3, 0
blr

; this function checks if a skipflag is set, always return true
.org 0x800bfd20
li r3, 1
blr

; don't treat faron statues differently after levias
.org 0x80142078
b 0x801420d8

; don't force open the map to faron after levias
.org 0x802d01e8
b 0x802d0214

; always just return from the function that wants to open
; the item screen after a treasure
.org 0x802d02c0
blr

; skip adding to the pouch counter, that is done in the events now
.org 0x80253be4
nop

; after giving gratitude crystals, update the counter in text number 1 with the crystal counter
.org 0x802539dc
b 0x802da0c0
; this replaces unused code
.org 0x802da0c0
lwz r3, -0x4040(r13) ;ITEMFLAG_MANAGER
li r4, 0x1F6 ;gratitude crystals
bl FlagManager__getUncommittedFlags
lwz r4, -0x3fb8(r13) ;LYT_MSG_MANAGER
lwz r4, 0x724(r4) ;Text Manager
stw r3, 0x8A0(r4) ;text counter 1
lwz r0, 0x24(r1) ; instruction that was overwritten
b 0x802539e0

; change storyflag that disables sandship dowsing
; to bombed sandship
.org 0x80097b18
li r4, 0x110

; when checking for story- & sceneflags for triggering the NpcTke
; check both flags to trigger, instead of only one
.org 0x8027ce64
cmplwi r3, 0
bne 0x8027ce78
li r3, 0
nop

; special text when entering faron pillar during SotH, skip over it
.org 0x80141f00
b 0x80141f44

.close

.open "d_a_obj_time_door_beforeNP.rel"
.org 0xD4C
blt 0xDEC
.close

.open "d_a_obj_door_bossNP.rel"
; fix the array of storyflags the boss door checks for as dungeon completion
.org 0x9770 ; 0x130+ data2 offset 0x9640
.word 0
.word 0x53
.word 0
.word 0x7
.word 0
.word 0x32C
.word 0
.word 0x288
.word 0
.word 0x54
.word 0
.word 0x3A5
.close

.open "d_a_obj_item_heart_containerNP.rel"
.org 0x2214 ; function, that gives the item when collecting the container
rlwinm r3,r0,16,24,31 ; r0 has params1, item is (r0 >> 16) & 0xFF
li r4, 7 ; set subtype to 7 to force textbox
.close

.open "d_a_obj_chandelierNP.rel"
.org 0x1D70 ; function, that spawns the "heartpiece"
li r4, 9 ; set itemsubtype to 9
stw r30, 0x8(r1)
mr r30, r3
lwz r0, 0x4(r3) ; params1
rlwinm r3,r0,24,24,31 ; (r0 >> 8) & 0xFF, itemid

.org 0xF7C ; in function, that runs when bonking the chandelier
nop ; branches over activating the chanderlier event after spiral charge, never take the branch
.close

.open "d_a_obj_soilNP.rel"
.org 0x1A10 ; function that gives the key piece
lbz r3, 0xa8(r31) ; grab itemid from first params2 byte

.org 0x1944 ; also function that gives the key piece
lbz r3, 0xa8(r3) ; grab itemid from first param2 byte

.org 0xE54 ; function that sets the sceneflag
nop ; normally only sets the flag, if the item was a key piece, nop the branch
.close

.open "d_a_obj_warpNP.rel"
.org 0x22C0 ; function, that gives the trial item
lbz r3, 0x4(r3) ; first byte of params1 is itemid
li r4, -1 ; set pouch slot param to -1, otherwise pouch items break
b 0x231C ; go directly to give item function call

lbz r0, 0xc7d(r3) ; inject code here to delay the warp out of the trial by one frame, load counter (was itemid originally, not needed anymore)
cmplwi r0, 0
bne 0x2354 ; if the frame passed, go trigger the event
addi r0, r0, 1 ; otherwise increment
stb r0, 0xc7d(r3) ; story counter back
b 0x23A4; jump to end of function that branched here

.org 0x2344 ; branch function, that triggers the walk out event to the code injected above
b 0x22D0

; the trial storyflags got changed, cause they used the same one as the items associated with it
.org 0x2F48
li r4, 0x397 ; new storyflag

.org 0x2F88
li r4, 0x398

.org 0x2FD4
li r4, 0x399

.org 0x3020
li r4, 0x39A

.org 0x2B08
li r4, 0x397

.org 0x2B48
li r4, 0x398

.org 0x2B94
li r4, 0x399

.org 0x2BE0
li r4, 0x39A

.org 0xC8C
li r4, 0x397

.org 0xCCC
li r4, 0x398

.org 0xD18
li r4, 0x399

.org 0xD64
li r4, 0x39A

.close

.open "d_a_e_bcNP.rel"
.org 0x4828 ; in death function (?), when it checks if it should set the sceneflag
lbz r0, 0x1254(r31) ; load if the boko has the key
cmplwi r0, 1 ; check if it's 1, it doesn't set the flag if this condition is met

.org 0x491C ; in the same functions, never drop the key on the boko death
b 0x4934

.org 0xE8F8 ; when whipping the bokoblin
lbz r3, 0xab(r31) ; load itemid from last byte of params2
lbz r6, 0x1343(r31)
extsb r30, r30
li r4, 9 ; change itemtype to 9

.org 0xEB30 ; another function that gives the item
lbz r3, 0xab(r31) ; load itemid from last byte of params2
lbz r6, 0x1343(r31)
extsb r28, r28
li r4, 9 ; change itemtype to 9

.close

.open "d_a_e_bceNP.rel"
.org 0x162C ; init func
li r3, 0 ; don't kill the hook beetle bokos if you have hook beetle
b 0x163C

.close

.open "d_a_obj_goddess_cubeNP.rel"
.org 0x1330 ; in function that checks, if the cube can be hit
li r3, 1 ; don't require the first cube near skyview
blr

.close

.open "d_a_npc_aqua_dragonNP.rel"
.org 0x3E7C ; function that checks AC flag
li r4, 0x384 ; new AC flag

.close

.open "d_a_npc_kyui_elderNP.rel"
.org 0x2A30 ; function that spawns slingshot
blr ; do not spawn it ever

.org 0x2CE0 ; let the function, that checks for the slingshot appears storyflag always return false
li r3, 0
blr

.org 0x2C80 ; function that checks for erla storyflag
li r3, 0 ; always return false
blr

.org 0x2E30 ; function that checks for slingshot itemflag
li r3, 0 ; always return false
blr

.close

.open "d_a_obj_chestNP.rel"
.org 0x14E4 ; in function that gives the item
li r4, -1 ; -1 for bottle slot, or pouch items break

.close

; TODO: bird speed patch

.open "d_a_birdNP.rel"
.org 0xA158
lfs f1, 0x18c(r30)

.org 0x9F9C
lfs f0, 0x18c(r4)
.close

.open "d_a_npc_dive_game_judgeNP.rel"
.org 0x2BD8 ; in function that checks if he should loose his party wheel
li r4, 0x130 ; always loose it (batreaux storyflag)

.close

.open "d_a_npc_desertrobotNP.rel"
.org 0x1EA4 ; checks here, if you have hook beetle so he puts it out of his hands, always make it true
nop

.close

.open "d_a_npc_terryNP.rel"
.org 0x444 ; always remove cage from beetles hands
nop

.close

.open "d_a_obj_blast_rockNP.rel"
.org 0xEA4 ; init func
li r0, 0xFF ; always set eventid to 0xFF (so none will play on exploding)

.close

.open "d_a_obj_pot_salNP.rel"
.org 0xB44 ; function that decides if the pot should spawn in lumpy pumpkin
nop ; do not need spiral charge to spawn

.close

.open "d_t_D3_scene_changeNP.rel"
.org 0x45C ; function, that loads the exit for skyloft
li r0, 0x34 ; use different entrance, to not softlock
.close
