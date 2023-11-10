#!/bin/sh

LOG_FILE=/home/jasl/phala/pruntime_1/pruntime.out
LOG_CMD="ssh -t poc5 'tail -f ${LOG_FILE} 2>/dev/null | grep event_chain'"
# LOG_CMD="tail -n100000 -f /home/kvin/codes/chain/phala/phala-blockchain/e2e/phala-e2e-2023-10-31T07:10:53.590Z-wbJHiM/pruntime0.log"
CONTRACT=0xdce4aabe72c9eb4e7985f7d3a1476d036781cdde451f7c1f6389ff8951f17c1e

cargo run --release -- --ticket-house-address ${CONTRACT} --event-log-emitter "${LOG_CMD}"
