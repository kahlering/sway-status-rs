#include <alsa/asoundlib.h>

#define DEVICE_NAME "default"
#define MIXER_NAME "Master"


static int setup(snd_mixer_t **mixer, snd_mixer_elem_t **elem, snd_mixer_selem_id_t **sid){
    if (snd_mixer_open(&*mixer, 0) < 0){
        return -1;
    }

    if (snd_mixer_attach(*mixer, DEVICE_NAME) < 0){
        return -1;
    }

    if (snd_mixer_selem_register(*mixer, NULL, NULL) < 0){
        return -1;
    }

    if (snd_mixer_load(*mixer) < 0){
        return -1;
    }

    snd_mixer_selem_id_alloca(&*sid);
    snd_mixer_selem_id_set_index(*sid, 0);
    snd_mixer_selem_id_set_name(*sid, MIXER_NAME);
    *elem = snd_mixer_find_selem(*mixer, *sid);
    if (!*elem){
        return -1;
    }

    if (snd_config_update_free_global() < 0){
        return -1;
    }
    return 0;
}

int get_audio_volume(void){
    snd_mixer_t *mixer;
    snd_mixer_elem_t *elem;
    snd_mixer_selem_id_t *sid;

    if(setup(&mixer, &elem, &sid) < 0){
        return -1;
    }

    int s;
    int r = snd_mixer_selem_get_playback_switch(elem, SND_MIXER_SCHN_UNKNOWN, &s);
    if(r < 0){
        return -1;
    }
    if(!s){
        return -2;
    }

    long minv, maxv;
    if(snd_mixer_selem_get_playback_volume_range(elem, &minv, &maxv) < 0){
        return -1;
    }

    long vol;
    if (snd_mixer_selem_get_playback_volume(elem, 0, &vol) < 0){
        return -1;
    }
    if(snd_mixer_close(mixer) < 0){
        return -1;
    }
    return (100* (vol - minv)) / (maxv - minv);
}

int set_audio_volume(int volume){
    snd_mixer_t *handle;
    snd_mixer_elem_t *elem;
    snd_mixer_selem_id_t *sid;

    if(setup(&handle, &elem, &sid) < 0){
        return -1;
    }

    long minv, maxv;
    if(snd_mixer_selem_get_playback_volume_range(elem, &minv, &maxv) < 0){
        return -1;
    }

    long val = ((maxv - minv) / 100) * volume + minv;
    if(snd_mixer_selem_set_playback_volume_all(elem, val) < 0){
        return -1;
    }

    if(snd_mixer_close(handle) < 0){
        return -1;
    }
    return 0;
}