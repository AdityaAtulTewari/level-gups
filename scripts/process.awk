#!/usr/bin/awk -f
BEGIN {print "---------------------------------- Useful Stats -----------------------------------------"}
$2 ~ /cycles/ {cycles += $1}
$2 ~ /instructions/ {instr += $1}
$2 ~ /walk_duration/ {walk_duration += $1}
$2 ~ /dtlb_l1/ {dtlb_l1 += $1}
$2 ~ /dtlb_l2/ {dtlb_l2 += $1}
$2 ~ /dtlb_l3/ {dtlb_l3 += $1}
$2 ~ /dtlb_me/ {dtlb_me += $1}
END {print "cycles\t" cycles}
END {print "instructions\t" instr}
END {print "walk_duration\t" walk_duration}
END {print "dtlb_l1\t" dtlb_l1}
END {print "dtlb_l2\t" dtlb_l2}
END {print "dtlb_l3\t" dtlb_l3}
END {print "dtlb_me\t" dtlb_me}
END {print "CPI\t" cycles/instr}
END {PRISM_E_WD = dtlb_l1 * 3 + dtlb_l2 * 11 + dtlb_l3 * 40 + dtlb_me * 200}
END {print "PRISM_Est_walk_duration\t" PRISM_E_WD}
END {print "PRISM_Est_WD_error_percent\t" (walk_duration - PRISM_E_WD)/walk_duration * 100}
END {print "walk_cycles_percent\t" walk_duration/cycles *100}
END {print "est_percent_mem_walk_duration\t" 200* dtlb_me/walk_duration *100}
END {print "est_percent_from_memory\t" dtlb_me * 200/cycles * 100}
