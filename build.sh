COMPILER="target/release/simple-compiler"
CC="gcc"

function comp {
    BN=$(basename -s .tb $1)
    TTOUTPUT=$(${COMPILER} -i $1 -o ${BN}.c 2>&1)
    if [ $? -ne 0 ]; then
        echo "${TTOUTPUT}"
    else
        CCOUTPUT=$(${CC} -o ${BN} ${BN}.c)
        if [ $? -ne 0 ]; then
            echo "${CCOUTPUT}"
        else
            echo "${TTOUTPUT}"
        fi
    fi
}

if [ $# -eq 0 ]; then
    for i in $(ls src/examples/code/source/*.tb); do
        comp $i
    done
else
    comp $1
fi
