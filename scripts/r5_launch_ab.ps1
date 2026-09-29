# Round-5 teacher A/B launcher (issue 009, bench 026).
# Minimal on purpose: the previous inline Start-Process call hung the agent
# tool session (the detached child held the session pipes) and the watchdog's
# tree-kill took the A/B down with it after cell 1 of 3. A launcher that
# RETURNS cleanly keeps the child alive — the trainer launches proved the
# pattern across a 4-hour run.
$exe = 'C:\Users\katop\AppData\Local\Temp\train_r5\release\deps\tetris_round5_teacher_ab-cf2db0da442a7c9a.exe'
$p = Start-Process -FilePath $exe `
    -WorkingDirectory 'E:\git\riir-instinct' `
    -WindowStyle Hidden `
    -ArgumentList '--model', '../riir-train/data/tetris_critic_r4p2/tetris_mlp_v1.bin' `
    -RedirectStandardOutput 'E:\git\riir-instinct\.raw\r5_ab.log' `
    -RedirectStandardError 'E:\git\riir-instinct\.raw\r5_ab.err' `
    -PassThru
Write-Output ("started pid {0}" -f $p.Id)
