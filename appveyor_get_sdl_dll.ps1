#Download SDL
Start-FileDownload "https://www.libsdl.org/release/SDL2-devel-2.0.5-VC.zip" -FileName "${env:Temp}\sdl2.zip"

Add-Type -AssemblyName System.IO.Compression.FileSystem
function Unzip
{
    param([string]$zipfile, [string]$outpath)

    [System.IO.Compression.ZipFile]::ExtractToDirectory($zipfile, $outpath)
}

Unzip "${env:Temp}\sdl2.zip" "${env:Temp}\sdl2"


if (-Not (Test-Path "msvc\lib\"))
{
     md -path "msvc\lib\"
}

Copy-Item "${env:Temp}\sdl2\SDL2-2.0.5\lib\*" "msvc\lib\"
function GetFiles($path = $pwd, [string[]]$exclude)
{
    foreach ($item in Get-ChildItem $path)
    {
        if ($exclude | Where {$item -like $_}) { continue }

        $item
        if (Test-Path $item.FullName -PathType Container)
        {
            GetFiles $item.FullName $exclude
        }
    }
}

GetFiles("msvc\lib\")