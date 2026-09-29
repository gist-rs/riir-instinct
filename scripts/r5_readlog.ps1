$log = 'E:\git\riir-instinct\.raw\r5_export.log'
$raw = [IO.File]::ReadAllText($log)
$clean = $raw -replace "`0", ''
Write-Output $clean
Write-Output '===DIR==='
Get-ChildItem 'E:\git\riir-instinct\.raw\tetris_critic' | ForEach-Object { Write-Output ("{0}`t{1}" -f $_.Length, $_.Name) }
