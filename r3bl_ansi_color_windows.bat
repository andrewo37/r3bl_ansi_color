@echo off
echo What would you like to do Red Leader?
echo.
echo 1. Run R3bl
echo 2. Build Clean
REM echo 3. Run R3bl with flamegraph
REM echo 4. Command

echo.
set /p input=Command:

REM Run
if %input%==1 (
	REM Run
	cargo run --example main
) else if %input%==2 (
	REM build clean
	cargo clean
	cargo +nightly update
	cargo build
REM ) else if %input%==3 (
REM ::Make new commands here::
REM )
) else (
echo Pull off!
)
