import logging
from sys import argv
log_level = logging.INFO
if '--debug' in argv:
 log_level = logging.DEBUG
logging.basicConfig(format='%(asctime)s - %(message)s')
logging.getLogger().setLevel(log_level)

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
status, result = read_config('my-test.kv', ['first-name', 'last-name'], {'middlename': ''})
if not status:
 logging.error(result)
 exit(1)
cfg = result
logging.info('your name is ' + cfg['first-name'] + '  ' + cfg['middle-name'] + '  ' + cfg['last-name'])

