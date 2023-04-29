#!/bin/bash -eu

TOWNS=400
BOX_SIZE=500
DIST=L2

for dim in 1 2 3 4 5 6 7 8;
do
    for seed in 1 3 5 7 9;
    do
        ./target/release/equilibrium_state \
            --seed ${seed} \
            --towns ${TOWNS} \
            --box-size ${BOX_SIZE} \
            --dist ${DIST} \
            --dim ${dim} \
            --temp-max 1000 > ${TOWNS}-${BOX_SIZE}-${dim}-${DIST}_${seed}.dat
    done
done
