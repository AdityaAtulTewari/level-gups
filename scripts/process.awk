#!/usr/bin/awk -f
BEGIN {print "---------------------------------- Useful Stats -----------------------------------------"}
$2 == "cycles" {cycles += $1}
$2 ~ /instructions/ {instr += $1}
$2 == "ept.walk_cycles" {ept_cycles += $1}
$2 ~ /walk_duration/ {walk_duration += $1}
$2 ~ /dtlb_l1/ {dtlb_l1 += $1}
$2 ~ /dtlb_l2/ {dtlb_l2 += $1}
$2 ~ /dtlb_l3/ {dtlb_l3 += $1}
$2 ~ /dtlb_me/ {dtlb_me += $1}
$2 == "L1" {L1 = $1; UnV = $3}
$2 == "L2" {L2 = $1}
$2 == "L3" {L3 = $1}
$2 == "LM" {LM = $1}
$2 == "DLP"{DLP = $1}
$2 == "AVE_COM" {AVE_COM = $1}
END {if (!L1) L1 =   3}
END {if (!L2) L2 =  11}
END {if (!L3) L3 =  40}
END {if (!LM) LM =(walk_duration - dtlb_l1*L1 -dtlb_l2*L2 - dtlb_l3*L3)/dtlb_me}
END {if (LM < 200) LM = 200}
END {PRISM_E_WD = dtlb_l1 * L1 + dtlb_l2 * L2 + dtlb_l3 * L3 + dtlb_me * LM}
END {print "cycles\t" cycles}
END {print "instructions\t" instr}
END {print "walk_duration\t" walk_duration}
END {print "ept_duration\t" ept_cycles}
END {print "EPTD_WD_percent\t" ept_cycles/walk_duration * 100}
END {print "PRISM_Est_walk_duration\t" PRISM_E_WD}
END {print "latency_vector\t" L1,L2,L3,LM}
END {print "unexplained_variance\t" UnV}
END {print "PRISM_Est_WD_error_percent\t" (walk_duration - PRISM_E_WD)/walk_duration * 100}
END {print "dtlb_l1\t" dtlb_l1}
END {print "dtlb_l2\t" dtlb_l2}
END {print "dtlb_l3\t" dtlb_l3}
END {print "dtlb_me\t" dtlb_me}
END {print "walk_cycles_percent\t" walk_duration/cycles *100}
END {print "PRISM_ep_mem_walk_duration\t" LM* dtlb_me/walk_duration *100}
END {print "PRISM_ep_mem_cycles\t" dtlb_me *LM /cycles * 100}
END {if (!DLP) DLP = 30}
END {if (!AVE_COM) AVE_COM = 1.01}
END {print "PIM_ep_saved_min\t" AVE_COM, LM+DLP, (dtlb_me * LM - dtlb_me/AVE_COM * (LM + DLP))/cycles * 100}
END {DLP = 0}
END {AVE_COM = 2}
END {print "PIM_ep_saved_max\t" AVE_COM, LM+DLP, (dtlb_me * LM - dtlb_me/AVE_COM * (LM + DLP))/cycles * 100}
