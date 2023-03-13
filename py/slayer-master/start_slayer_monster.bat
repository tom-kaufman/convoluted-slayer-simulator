call "C:\Users\Tom\.conda\envs\convoluted-slayer-simulator\Lib\venv\scripts\nt\activate.bat"
cd "C:\rust\convoluted-slayer-simulator\py\slayer-master"
"C:\Users\Tom\.conda\envs\convoluted-slayer-simulator\Scripts\waitress-serve.exe" --host 127.0.0.1 --port 5001 --call slayer_master:create_app
call "C:\Users\Tom\.conda\envs\convoluted-slayer-simulator\Lib\venv\scripts\nt\deactivate.bat"
pause