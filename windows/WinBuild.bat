@echo off
setlocal EnableExtensions
setlocal EnableDelayedExpansion

reg query "HKLM\Hardware\Description\System\CentralProcessor\0" | find /i "x86" > NUL && set OS=32BIT || set OS=64BIT
set RUSTFLAGS=-Ctarget-feature=+crt-static -Clink-args=/subsystem:console,5.01

if %OS% == 64BIT (
    set vswhere="%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe"
) else (
    set vswhere="%ProgramFiles%\Microsoft Visual Studio\Installer\vswhere.exe"
)

if exist %vswhere% (
    for /f "usebackq delims=" %%i in (`%vswhere% -prerelease -latest -property installationPath`) do (
        for /f "tokens=2* skip=2" %%a in ('reg query "HKCU\Environment" /v "Path"') do (
            set "PATH_EXTR=%%b"
        )

        if "x!PATH_EXTR:%%i\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin=%!" == "x!PATH_EXTR!" (
            set "PATH_NEW=!PATH_EXTR!;%%i\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin"
            reg add "HKCU\Environment" /v Path /d "!PATH_NEW!" /f
        )

        if "x!PATH:%%i\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin=%!" == "x!PATH!" (
            call "%~dp0\RefreshEnv.cmd"
        )

        call "%%i\Common7\Tools\VsMSBuildCmd.bat" -no_logo

        pushd "%~dp0\.."
        cargo build --release --target i686-pc-windows-msvc
        popd

        pushd "%~dp0\..\asm6502"
        cargo build --release --target i686-pc-windows-msvc
        popd

        goto :EOF
    )
)

echo "Visual Studio is required to build the project"
