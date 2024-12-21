from subprocess import Popen, PIPE

# region: dev
from rust_build import build_exe
exepath = build_exe()
# endregion
process = Popen([exepath], stdin=PIPE, stdout=PIPE, text=True)

stdin_write = process.stdin.write
stdin_flush = process.stdin.flush
stdout_readline = process.stdout.readline

line = stdout_readline().strip()
assert(line == "ready")

INITIALIZED = False

def init(obs):
    global INITIALIZED
    stdin_write(obs.board + "\n")
    stdin_flush()
    line = stdout_readline().strip()
    INITIALIZED = True
    return line

def move(obs):
    stdin_write(obs.lastMove + "\n")
    stdin_flush()
    return stdout_readline().strip()

def botly(obs):
    if not INITIALIZED:
        return init(obs)
    else:
        return move(obs)