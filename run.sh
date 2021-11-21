## RUN SCRIPT FOR DSA CPP FILES

## USE: sh run.sh -f src/[FILENAME]

while getopts f: option
do
case "${option}"
in
f) FILE=${OPTARG};;
esac
done

rustc $FILE -o out.out
./out.out
rm out.out

