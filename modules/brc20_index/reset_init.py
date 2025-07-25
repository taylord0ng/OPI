# pip install python-dotenvAdd commentMore actions
# pip install psycopg2-binary
# pip install stdiomask

import os, psycopg2, stdiomask
from dotenv import load_dotenv, dotenv_values

init_env = True

# does .env file exist?
if os.path.isfile('.env'):
  res = input("Do you want to re-initialise .env file? (y/n) ")
  if res != 'y':
    init_env = False

if init_env:
  DB_USER="postgres"
  DB_HOST="localhost"
  DB_PORT="5432"
  DB_DATABASE="postgres"
  DB_PASSWD=""
  DB_SSL="false"
  NETWORK_TYPE="mainnet"
  REPORT_TO_INDEXER="true"
  REPORT_URL="https://api.opi.network/report_block"
  REPORT_RETRIES="10"
  REPORT_NAME="opi_brc20_index"
  BRC20_PROG_ENABLED="false"
  BRC20_PROG_RPC_URL="http://localhost:18545"
  BRC20_PROG_RPC_SERVER_USER="user"
  BRC20_PROG_RPC_SERVER_PASSWORD="password"
  BRC20_PROG_BALANCE_SERVER_URL="localhost:18546"

  print("Initialising .env file")
  print("leave blank to use default values")
  use_other_env = False
  other_env_exists = os.path.isfile('../brc20_api/.env')
  if other_env_exists:
    res = input(".env on brc20_api already exists, do you want to use values from there? (y/n) ")
    if res == 'y':
      use_other_env = True
  if use_other_env:
    env = dotenv_values(dotenv_path='../brc20_api/.env')
    DB_USER = env.get("DB_USER") or "postgres"
    DB_HOST = env.get("DB_HOST") or "localhost"
    DB_PORT = env.get("DB_PORT") or "5432"
    DB_DATABASE = env.get("DB_DATABASE") or "postgres"
    DB_PASSWD = env.get("DB_PASSWD")
    DB_SSL = env.get("DB_SSL") or "false"
  else:
    res = input("BRC20 Postgres DB username (Default: postgres): ")
    if res != '':
      DB_USER = res
    res = input("BRC20 Postgres DB host (Default: localhost) leave default if postgres is installed on the same machine: ")
    if res != '':
      DB_HOST = res
    res = input("BRC20 Postgres DB port (Default: 5432): ")
    if res != '':
      DB_PORT = res
    res = input("BRC20 Postgres DB name (Default: postgres) leave default if no new dbs are created: ")
    if res != '':
      DB_DATABASE = res
    res = stdiomask.getpass("BRC20 Postgres DB password: ")
    DB_PASSWD = res
    res = input("BRC20 Postgres DB SSL (Default: false) options: true, false: ")
    if res != '':
      DB_SSL = res
  res = input("Network type (Default: mainnet) options: mainnet, testnet, testnet4, signet, regtest: ")
  if res != '':
    NETWORK_TYPE = res
  res = input("Report to main indexer (Default: true): ")
  if res != '':
    REPORT_TO_INDEXER = res
  if REPORT_TO_INDEXER == 'true':
    res = input("Report URL (Default: https://api.opi.network/report_block): ")
    if res != '':
      REPORT_URL = res
    res = input("Report retries (Default: 10): ")
    if res != '':
      REPORT_RETRIES = res
    while True:
      res = input("Report name: ")
      if res != '':
        REPORT_NAME = res
        break
      else:
        print('Report name cannot be empty')
  res = input("Enable BRC20 programmable module (Default: false): ")
  if res != '':
    BRC20_PROG_ENABLED = res
  if BRC20_PROG_ENABLED == 'true':
    res = input("BRC20 programmable module RPC URL (Default: http://localhost:18545): ")
    if res != '':
      BRC20_PROG_RPC_URL = res
    res = input("BRC20 programmable module RPC username (Default: user): ")
    if res != '':
      BRC20_PROG_RPC_SERVER_USER = res
    res = stdiomask.getpass("BRC20 programmable module RPC password (Default: password): ")
    if res != '':
      BRC20_PROG_RPC_SERVER_PASSWORD = res
    res = input("BRC20 programmable module balance server URL (Default: localhost:18546): ")
    if res != '':
      BRC20_PROG_BALANCE_SERVER_URL = res

  f = open('.env', 'w')
  f.write('DB_USER="' + DB_USER + '"\n')
  f.write('DB_HOST="' + DB_HOST + '"\n')
  f.write('DB_PORT="' + DB_PORT + '"\n')
  f.write('DB_DATABASE="' + DB_DATABASE + '"\n')
  f.write('DB_PASSWD="' + DB_PASSWD + '"\n')
  f.write('DB_SSL="' + DB_SSL + '"\n')
  f.write('NETWORK_TYPE="' + NETWORK_TYPE + '"\n')
  f.write('REPORT_TO_INDEXER="' + REPORT_TO_INDEXER + '"\n')
  f.write('REPORT_URL="' + REPORT_URL + '"\n')
  f.write('REPORT_RETRIES="' + REPORT_RETRIES + '"\n')
  f.write('REPORT_NAME="' + REPORT_NAME + '"\n')
  f.write('BRC20_PROG_ENABLED="' + BRC20_PROG_ENABLED + '"\n')
  f.write('BRC20_PROG_RPC_URL="' + BRC20_PROG_RPC_URL + '"\n')
  f.write('BRC20_PROG_RPC_SERVER_USER="' + BRC20_PROG_RPC_SERVER_USER + '"\n')
  f.write('BRC20_PROG_RPC_SERVER_PASSWORD="' + BRC20_PROG_RPC_SERVER_PASSWORD + '"\n')
  f.write('BRC20_PROG_BALANCE_SERVER_URL="' + BRC20_PROG_BALANCE_SERVER_URL + '"\n')
  f.close()

res = input("Are you sure you want to initialise/reset the brc20 database? (y/n) ")
if res != 'y':
  print('aborting')
  exit(1)

load_dotenv()
db_user = os.getenv("DB_USER") or "postgres"
db_host = os.getenv("DB_HOST") or "localhost"
db_port = int(os.getenv("DB_PORT") or "5432")
db_database = os.getenv("DB_DATABASE") or "postgres"
db_password = os.getenv("DB_PASSWD")

## connect to db
conn = psycopg2.connect(
  host=db_host,
  port=db_port,
  database=db_database,
  user=db_user,
  password=db_password)
conn.autocommit = True
cur = conn.cursor()

db_exists = False
try:
  cur.execute('select count(*) from brc20_block_hashes;')
  hash_cnt = cur.fetchone()[0]
  if hash_cnt > 0:
    db_exists = True
except:
  pass

if db_exists:
  res = input("It seems like you have entries on DB, are you sure to reset databases? This WILL RESET indexing progress. (y/n) ")
  if res != 'y':
    print('aborting')
    exit(1)

## reset db
sqls = open('src/database/sql/db_reset.sql', 'r').read().split(';')
for sql in sqls:
  if sql.strip() != '':
    cur.execute(sql)

sqls = open('src/database/sql/db_init.sql', 'r').read().split(';')
for sql in sqls:
  if sql.strip() != '':
    cur.execute(sql)

## close db
cur.close()
conn.close()

print('done')

def sanitise(value):
    """Sanitise a string for safe output."""
    return value.replace("/", "%2F").replace(":", "%3A").replace("@", "%40")

## print DATABASE_URL
print("Run the following command to set DATABASE_URL in your environment before compiling:")
print(f"export DATABASE_URL=postgresql://{sanitise(db_user)}:{sanitise(db_password)}@{sanitise(db_host)}:{db_port}/{sanitise(db_database)}")