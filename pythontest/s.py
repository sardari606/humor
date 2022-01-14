import signal
var = ''
def handler(sig, frame):
 print(var)
signal.signal(signal.SIGALRM, handler) #register alarm to function
var = input("pls put the input: ")
i = 1
while i != 0:
 signal.alarm(1)
 signal.pause()
 i += 1
