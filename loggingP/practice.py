import logging
import yaml
import signal
from sys import argv
log_level = logging.INFO
if '--debug' in argv:
 log_level = logging.DEBUG
logging.basicConfig(format='%(asctime)s - %(message)s')
logging.getLogger().setLevel(log_level)
def signalhandle1(signum, frame):
  log_level = logging.DEBUG
def signalhandle2(signum, frame):
  log_level = logging.WARNING
def sigrap(signum, stack):
  None
signal.signal(signal.SIGUSR1, signalhandle1)
signal.signal(signal.SIGUSR2, signalhandle2)
signal.signal(signal.SIGALRM, sigrap)
while 1:
  signal.alarm(1)
  signal.pause()
def read(filename, mandatorykeylist, cfg):
 for line in open(filename):
  logging.debug('->' + line[:-1])
  if line.strip() == '':
    continue
  left, right = line.split('=', 1)
  logging.debug('left' + left + ' right' + right[:-1])
  key, value = left.strip(), right.strip()
  logging.debug('key'+ key +' value'+ value)
  cfg[key] = value
 for mandatory in mandatorykeylist:
  if mandatory not in cfg.keys():
   return False, 'could not found " ' + mandatory + ' " '
 return True, cfg
status, result = read.config('my-test.kv', ['first-name', 'last-name'], {'middlename': ''})
if not status:
 logging.warning(result)
 exit(1)
cfg = result
logging.info('your name is ' + cfg['first-name'] + '  ' + cfg['middle-name'] + '  ' + cfg['last-name'])

