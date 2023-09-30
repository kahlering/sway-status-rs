#include <stdlib.h>

void initialize_alsa_lib(const char *mix_name, const char *card);

void update_volume();
long get_audio_volume();
static long calculate_audio_volume();


int main(){
    initialize_alsa_lib("Speaker", "hw:0");


    while(1){
        update_volume();
        long vol = get_audio_volume();
        printf("vol %d\n", vol);
        sleep(1);
    }


}