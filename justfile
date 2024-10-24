clean:
  cargo clean

test:
  anchor test

build:
  anchor build

keys:
  anchor keys list

deploy:
  anchor deploy

run-test-validator:
  solana-test-validator
