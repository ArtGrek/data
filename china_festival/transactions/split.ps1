$input = "bet_1_line_25_61d03a7d4e3c415fa74913b009f9a393.json"
$inputPath = (Resolve-Path $input).Path
$dir  = Split-Path $inputPath
$base = [System.IO.Path]::GetFileNameWithoutExtension($inputPath)
$ext  = [System.IO.Path]::GetExtension($inputPath)
$max  = 20000
$i    = 0
$j    = 1
$file = $null

Get-Content $inputPath | ForEach-Object {
    if (($i % $max) -eq 0) {
        if ($file) { $file.Close() }
        $filename = Join-Path $dir ("${base}_part${j}${ext}")
        $file = [System.IO.StreamWriter]::new($filename)
        $j++
    }
    $file.WriteLine($_)
    $i++
}
if ($file) { $file.Close() }
Remove-Item $inputPath