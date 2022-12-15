CC=clang
CFLAGS= -g -Wall -std=c17

all: main.o device.o instructions.o
	$(CC) -o main $(CFLAGS) main.o instructions.o device.o

main.o: main.c
	$(CC) -c main.c $(CFLAGS) -o main.o

instructions.o: instructions.c
	$(CC) -c instructions.c $(CFLAGS) -o instructions.o

device.o: device.c
	$(CC) -c device.c $(CFLAGS) -o device.o

.PHONY: clean
clean:
	rm *.o
	rm chip-8