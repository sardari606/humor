import signal
var = ''
def handler(sig, frame):
 print(var)
signal.signal(signal.SIGALRM, handler) #register alarm to function
var = input("pls put the input: ")
while 1:
 signal.alarm(1)
 signal.pause()
