Extract CSV parse reports from old-school World of Warcraft private server log web interface.

## Installing

⚠️ This script will run as root so please [inspect it](https://github.com/Kruhlmann/WWSParseExtractor/blob/master/install) before running. Alternatively, or if you're using Windows, you can download the JAR file directly from the [releases page](https://github.com/Kruhlmann/WWSParseExtractor/releases).

    curl https://raw.githubusercontent.com/Kruhlmann/WWSParseExtractor/master/install | sudo sh

## Running

Example exection:

    wpe http://sisuguild.fi/~juha/wws/20070610-magtheridon/ /tmp/

Produces the following stdout:

    Wrote file /tmp/rog_20070610-magtheridon_Powerforward.csv
    Wrote file /tmp/wrl_20070610-magtheridon_Quadragan.csv
    Wrote file /tmp/pri_20070610-magtheridon_Finl.csv
    Wrote file /tmp/rog_20070610-magtheridon_Leni.csv
    Wrote file /tmp/mag_20070610-magtheridon_Farlon.csv
    Wrote file /tmp/rog_20070610-magtheridon_Pehu.csv
    Wrote file /tmp/mag_20070610-magtheridon_Zabang.csv
    Wrote file /tmp/sha_20070610-magtheridon_Blitzkriegs.csv
    Wrote file /tmp/hnt_20070610-magtheridon_Chmee.csv
    Wrote file /tmp/mag_20070610-magtheridon_Ashie.csv
    Wrote file /tmp/drd_20070610-magtheridon_Archsin.csv
    Wrote file /tmp/drd_20070610-magtheridon_Iceran.csv
    Wrote file /tmp/wrl_20070610-magtheridon_Imbacoil.csv
    Wrote file /tmp/hnt_20070610-magtheridon_Barag.csv
    Wrote file /tmp/war_20070610-magtheridon_Zendog.csv
    Wrote file /tmp/drd_20070610-magtheridon_Aktivisti.csv
    Wrote file /tmp/war_20070610-magtheridon_Svebe.csv
    Wrote file /tmp/drd_20070610-magtheridon_Axu.csv
    Wrote file /tmp/pal_20070610-magtheridon_Nekograd.csv
    Wrote file /tmp/sha_20070610-magtheridon_Xenicc.csv
    Wrote file /tmp/sha_20070610-magtheridon_Deluvia.csv

    Process finished with exit code 0



Example output file:

    SPELL_NAME;TOTAL_DMG;CRIT_PCT;MISS_PCT;RESIST_PCT;HIT_COUNT;HIT_AVG_DMG;HIT_MAX_DMG;DOT_TICKS_COUNT;AVG_DOT_TICK_DMG;MAX_DOT_TICK_DMG;CRIT_COUNT;CRIT_AVG_DMG;CRIT_MAX_DMG;
    Melee;0;0.25;0.0;0.0;434;360;1340;0;0;0;221;729;2452
    Melee (glancing);0;0.0;0.0;0.0;225;275;906;0;0;0;0;0;0
    Sinister Strike;0;0.21;0.0;0.0;123;671;1460;0;0;0;33;1574;2981
    Deadly Poison VII (Nature);0;0.0;0.0;0.0;0;0;0;179;207;567;0;0;0
    Rupture (Physical);0;0.0;0.0;0.0;0;0;0;128;276;565;0;0;0
    Shiv;0;0.22;0.0;0.0;25;339;398;0;0;0;7;810;1021
    Eviscerate;0;0.17;0.0;0.0;5;1413;2466;0;0;0;1;2400;2400
    Garrote (Physical);0;0.0;0.0;0.0;0;0;0;12;280;289;0;0;0
