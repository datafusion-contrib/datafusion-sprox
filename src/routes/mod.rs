pub mod status;

#[get("/")]
pub fn index() -> &'static str {
    "MMMMMMMMXx:.  .xWMMMMMMX:   .:OWMMMMMMMM
MMMMMMMMO.     .dXNNNN0:      lWMMMMMMMM
MMMMMMMMNl       .''''.      '0MMMMMMMMM
MMMMMMMMWx.                  ,0WMMMMMMMM
Xxox0XN0l.     ..,;::;,..     .lO0kdc:dX
c    ...    .:x0NWMMMMWN0x:.     .     c
           :0WMMMMMMMMMMMMW0:.          
:'.      .oNMMMMMMMMMMMMMMMMNo.      .'c
WN0o.    :XMMMMMMMMMMMMMMMMMMX:    .d0NM
MMMWl    dWMMMMMMMMMMMMMMMMMMWd    lWMMM
MMMXc    lWMMMMMMMMMMMMMMMMMMWl    :XMMM
Xkl,     'OMMMMMMMMMMMMMMMMMMO'     'cxK
,         ,OWMMMMMMMMMMMMMMWO,         '
:          .cONMMMMMMMMMMNOc.          :
Kc..,col,    .'cdxOOOOxdc'.    'oxdl:,lK
MWXXWMMMNx'                  .oXMMMMWWWM
MMMMMMMMMNc                  .kMMMMMMMMM
MMMMMMMMMk.      :xkkkkl.     ;XMMMMMMMM
MMMMMMMMWx.     lNMMMMMWk.   .cKMMMMMMMM
MMMMMMMMMNOl.  ,KMMMMMMMWx,,lONMMMMMMMMM"
}
