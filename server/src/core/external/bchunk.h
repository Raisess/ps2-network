#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <unistd.h>

#define CUELLEN 1024
#define SECTLEN 2352

#define WAV_RIFF_HLEN 12
#define WAV_FORMAT_HLEN 24
#define WAV_DATA_HLEN 8
#define WAV_HEADER_LEN WAV_RIFF_HLEN + WAV_FORMAT_HLEN + WAV_DATA_HLEN

/*
 *	Ugly way to convert integers to little-endian format.
 *	First let netinet's hton() functions decide if swapping should
 *	be done, then convert back.
 */

#include <inttypes.h>
#ifdef _WIN32
#include <winsock2.h>
#else
#include <netinet/in.h>
#endif

#define bswap_16(x) \
     ((((x) >> 8) & 0xff) | (((x) & 0xff) << 8))
#define bswap_32(x) \
     ((((x) & 0xff000000) >> 24) | (((x) & 0x00ff0000) >>  8) |  \
      (((x) & 0x0000ff00) <<  8) | (((x) & 0x000000ff) << 24))

#define htoles(x) bswap_16(htons(x))
#define htolel(x) bswap_32(htonl(x))

struct track_t {
	int num;
	int mode;
	int audio;
	char *modes;
	char *extension;
	int bstart;
	int bsize;
	long startsect;
	long stopsect;
	long start;
	long stop;
	struct track_t *next;
};

int bchunk(char*, char*, char*);
