<template>
  <div class="index" @click="clear_button()"> 
    <!--header-->
    <div class="header">
      <img class="logo" src="@/assets/logo.png">
      <div class="nav" @click.stop>
        <div class="nav-item" @click="to_stake()" v-if="login">STAKE</div>
        <div class="nav-item" @click="sign_out()" v-if="login">SIGNOUT</div> 
        <div class="nav-item" @click="sign_in()" v-if="!login">SIGNIN</div>
      </div>
    </div>
    <!--content-->
    <div class="index-wrapper">
      <div class="line line-one">
        <div class="left">
          <div class="wheel">
            <div class="place-bet" ref="board">
              <svg viewBox="0 0 380 380"  class="spin-ani">
                <g>
                  <circle stroke-width="10" stroke="black" stroke-opacity="0.2" fill="transparent" cx="190" cy="190" r="182">
                  </circle>
                  <circle stroke="#ec0230" fill="transparent" stroke-width="68" cx="190" cy="190" r="146">
                  </circle>
                <g>
                <polygon points="173.8,10 206.2,10 200.8,79 179.2,79" fill="#3ab64b"></polygon>
                <polygon points="173.8,10 206.2,10 200.8,79 179.2,79" fill="#2b2627" transform="rotate(19.5,190,190)"></polygon>
                <polygon points="173.8,10 206.2,10 200.8,79 179.2,79" fill="#2b2627" transform="rotate(38.9,190,190)"></polygon>
                <polygon points="173.8,10 206.2,10 200.8,79 179.2,79" fill="#2b2627" transform="rotate(58.4,190,190)"></polygon>
                <polygon points="173.8,10 206.2,10 200.8,79 179.2,79" fill="#2b2627" transform="rotate(77.8,190,190)"></polygon>
                <polygon points="173.8,10 206.2,10 200.8,79 179.2,79" fill="#2b2627" transform="rotate(97.3,190,190)"></polygon>
                <polygon points="173.8,10 206.2,10 200.8,79 179.2,79" fill="#2b2627" transform="rotate(116.8,190,190)"></polygon>
                <polygon points="173.8,10 206.2,10 200.8,79 179.2,79" fill="#2b2627" transform="rotate(136.2,190,190)"></polygon>
                <polygon points="173.8,10 206.2,10 200.8,79 179.2,79" fill="#2b2627" transform="rotate(155.7,190,190)"></polygon>
                <polygon points="173.8,10 206.2,10 200.8,79 179.2,79" fill="#2b2627" transform="rotate(175.1,190,190)"></polygon>
                <polygon points="173.8,10 206.2,10 200.8,79 179.2,79" fill="#2b2627" transform="rotate(194.6,190,190)"></polygon>
                <polygon points="173.8,10 206.2,10 200.8,79 179.2,79" fill="#2b2627" transform="rotate(214.1,190,190)"></polygon>
                <polygon points="173.8,10 206.2,10 200.8,79 179.2,79" fill="#2b2627" transform="rotate(233.5,190,190)"></polygon>
                <polygon points="173.8,10 206.2,10 200.8,79 179.2,79" fill="#2b2627" transform="rotate(253.0,190,190)"></polygon>
                <polygon points="173.8,10 206.2,10 200.8,79 179.2,79" fill="#2b2627" transform="rotate(272.4,190,190)"></polygon>
                <polygon points="173.8,10 206.2,10 200.8,79 179.2,79" fill="#2b2627" transform="rotate(291.9,190,190)"></polygon>
                <polygon points="173.8,10 206.2,10 200.8,79 179.2,79" fill="#2b2627" transform="rotate(311.4,190,190)"></polygon>
                <polygon points="173.8,10 206.2,10 200.8,79 179.2,79" fill="#2b2627" transform="rotate(330.8,190,190)"></polygon>
                <polygon points="173.8,10 206.2,10 200.8,79 179.2,79" fill="#2b2627" transform="rotate(350.3,190,190)"></polygon>
                </g>
                <g>
                  <text x="190" y="32" transform="rotate(0.0,190,190)">0</text>
                  <text x="190" y="32" transform="rotate(9.7,190,190)">32</text>
                  <text x="190" y="32" transform="rotate(19.5,190,190)">15</text>
                  <text x="190" y="32" transform="rotate(29.2,190,190)">19</text>
                  <text x="190" y="32" transform="rotate(38.9,190,190)">4</text>
                  <text x="190" y="32" transform="rotate(48.7,190,190)">21</text>
                  <text x="190" y="32" transform="rotate(58.4,190,190)">2</text>
                  <text x="190" y="32" transform="rotate(68.1,190,190)">25</text>
                  <text x="190" y="32" transform="rotate(77.8,190,190)">17</text>
                  <text x="190" y="32" transform="rotate(87.6,190,190)">34</text>
                  <text x="190" y="32" transform="rotate(97.3,190,190)">6</text>
                  <text x="190" y="32" transform="rotate(107.0,190,190)">27</text>
                  <text x="190" y="32" transform="rotate(116.8,190,190)">13</text>
                  <text x="190" y="32" transform="rotate(126.5,190,190)">36</text>
                  <text x="190" y="32" transform="rotate(136.2,190,190)">11</text>
                  <text x="190" y="32" transform="rotate(146.0,190,190)">30</text>
                  <text x="190" y="32" transform="rotate(155.7,190,190)">8</text>
                  <text x="190" y="32" transform="rotate(165.4,190,190)">23</text>
                  <text x="190" y="32" transform="rotate(175.1,190,190)">10</text>
                  <text x="190" y="32" transform="rotate(184.9,190,190)">5</text>
                  <text x="190" y="32" transform="rotate(194.6,190,190)">24</text>
                  <text x="190" y="32" transform="rotate(204.3,190,190)">16</text>
                  <text x="190" y="32" transform="rotate(214.1,190,190)">33</text>
                  <text x="190" y="32" transform="rotate(223.8,190,190)">1</text>
                  <text x="190" y="32" transform="rotate(233.5,190,190)">20</text>
                  <text x="190" y="32" transform="rotate(243.3,190,190)">14</text>
                  <text x="190" y="32" transform="rotate(253.0,190,190)">31</text>
                  <text x="190" y="32" transform="rotate(262.7,190,190)">9</text>
                  <text x="190" y="32" transform="rotate(272.4,190,190)">22</text>
                  <text x="190" y="32" transform="rotate(282.2,190,190)">18</text>
                  <text x="190" y="32" transform="rotate(291.9,190,190)">29</text>
                  <text x="190" y="32" transform="rotate(301.6,190,190)">7</text>
                  <text x="190" y="32" transform="rotate(311.4,190,190)">28</text>
                  <text x="190" y="32" transform="rotate(321.1,190,190)">12</text>
                  <text x="190" y="32" transform="rotate(330.8,190,190)">35</text>
                  <text x="190" y="32" transform="rotate(340.6,190,190)">3</text>
                  <text x="190" y="32" transform="rotate(350.3,190,190)">26</text>
                </g>
                <circle stroke-width="25" stroke="black" stroke-opacity="0.2" fill="transparent" cx="190" cy="190" r="116"></circle></g>
                <g id="ball" class="ball" style="transform-origin: 190px 190px;">
                  <g transform="translate(178,48)">
                    <circle opacity=".4" fill="#231F20" cx="13" cy="15" r="13.5"></circle>
                    <circle fill="#FFF" cx="13" cy="13" r="12.5"></circle>
                    <path fill="#D1D3D4" d="M20.867 5.825c4.151 4.153 4.153 10.889.002 15.045-4.151 4.155-10.887 4.156-15.039.004-4.153-4.154 3.368-3.368 7.517-7.525 4.152-4.156 3.367-11.678 7.52-7.524z"></path><path fill="#FFF" d="M23.5 15.5a2 2 0 11-4 0 2 2 0 014 0zm-11.5 3a1.5 1.5 0 11-2.999.001A1.5 1.5 0 0112 18.5z"></path><circle fill="#FFF" cx="18.5" cy="8.5" r="1"></circle>
                  </g>
                </g>
              </svg>
            </div>
            <div class="wheel-base">
              <svg viewBox="0 0 380 380"><circle class="text" cx="190" cy="190" r="52" stroke-width="0" fill="rgba(0,0,0,0.2)"></circle><text text-anchor="middle" width="380" x="190" y="195" font-size="90px" stroke-width="10px">{{remain_time}}</text></svg>
            </div>
          </div>
        </div>
        <div class="right">
          <div class="board">
            <div class="board-wrap">
              <div class="board">
                <div id="n50" style="grid-area: n50 / n50 / n50 / n50;"><span>0</span></div>
                <div id="n51" class="red" style="grid-area: n51 / n51 / n51 / n51;"><span>1</span></div>
                <div id="n52" class="black" style="grid-area: n52 / n52 / n52 / n52;"><span>2</span></div>
                <div id="n53" class="red" style="grid-area: n53 / n53 / n53 / n53;"><span>3</span></div>
                <div id="n54" class="black" style="grid-area: n54 / n54 / n54 / n54;"><span>4</span></div>
                <div id="n55" class="red" style="grid-area: n55 / n55 / n55 / n55;"><span>5</span></div>
                <div id="n56" class="black" style="grid-area: n56 / n56 / n56 / n56;"><span>6</span></div>
                <div id="n57" class="red" style="grid-area: n57 / n57 / n57 / n57;"><span>7</span></div>
                <div id="n58" class="black" style="grid-area: n58 / n58 / n58 / n58;"><span>8</span></div>
                <div id="n59" class="red" style="grid-area: n59 / n59 / n59 / n59;"><span>9</span></div>
                <div id="n510" class="black" style="grid-area: n510 / n510 / n510 / n510;"><span>10</span></div>
                <div id="n511" class="black" style="grid-area: n511 / n511 / n511 / n511;"><span>11</span></div>
                <div id="n512" class="red" style="grid-area: n512 / n512 / n512 / n512;"><span>12</span></div>
                <div id="n513" class="black" style="grid-area: n513 / n513 / n513 / n513;"><span>13</span></div>
                <div id="n514" class="red" style="grid-area: n514 / n514 / n514 / n514;"><span>14</span></div>
                <div id="n515" class="black" style="grid-area: n515 / n515 / n515 / n515;"><span>15</span></div>
                <div id="n516" class="red" style="grid-area: n516 / n516 / n516 / n516;"><span>16</span></div>
                <div id="n517" class="black" style="grid-area: n517 / n517 / n517 / n517;"><span>17</span></div>
                <div id="n518" class="red" style="grid-area: n518 / n518 / n518 / n518;"><span>18</span></div>
                <div id="n519" class="red" style="grid-area: n519 / n519 / n519 / n519;"><span>19</span></div>
                <div id="n520" class="black" style="grid-area: n520 / n520 / n520 / n520;"><span>20</span></div>
                <div id="n521" class="red" style="grid-area: n521 / n521 / n521 / n521;"><span>21</span></div>
                <div id="n522" class="black" style="grid-area: n522 / n522 / n522 / n522;"><span>22</span></div>
                <div id="n523" class="red" style="grid-area: n523 / n523 / n523 / n523;"><span>23</span></div>
                <div id="n524" class="black" style="grid-area: n524 / n524 / n524 / n524;"><span>24</span></div>
                <div id="n525" class="red" style="grid-area: n525 / n525 / n525 / n525;"><span>25</span></div>
                <div id="n526" class="black" style="grid-area: n526 / n526 / n526 / n526;"><span>26</span></div>
                <div id="n527" class="red" style="grid-area: n527 / n527 / n527 / n527;"><span>27</span></div>
                <div id="n528" class="black" style="grid-area: n528 / n528 / n528 / n528;"><span>28</span></div>
                <div id="n529" class="black" style="grid-area: n529 / n529 / n529 / n529;"><span>29</span></div>
                <div id="n530" class="red" style="grid-area: n530 / n530 / n530 / n530;"><span>30</span></div>
                <div id="n531" class="black" style="grid-area: n531 / n531 / n531 / n531;"><span>31</span></div>
                <div id="n532" class="red" style="grid-area: n532 / n532 / n532 / n532;"><span>32</span></div>
                <div id="n533" class="black" style="grid-area: n533 / n533 / n533 / n533;"><span>33</span></div>
                <div id="n534" class="red" style="grid-area: n534 / n534 / n534 / n534;"><span>34</span></div>
                <div id="n535" class="black" style="grid-area: n535 / n535 / n535 / n535;"><span>35</span></div>
                <div id="n536" class="red" style="grid-area: n536 / n536 / n536 / n536;"><span>36</span></div>
                <div id="n10" style="grid-area: n10 / n10 / n10 / n10;"><span>2:1</span></div>
                <div id="n11" style="grid-area: n11 / n11 / n11 / n11;"><span>2:1</span></div>
                <div id="n12" style="grid-area: n12 / n12 / n12 / n12;"><span>2:1</span></div>
                <div id="n20" style="grid-area: n20 / n20 / n20 / n20;"><span>1ST 12</span></div>
                <div id="n21" style="grid-area: n21 / n21 / n21 / n21;"><span>2ND 12</span></div>
                <div id="n22" style="grid-area: n22 / n22 / n22 / n22;"><span>3RD 12</span></div>
                <div id="n30" style="grid-area: n30 / n30 / n30 / n30;"><span>1-18</span></div>
                <div id="n40" style="grid-area: n40 / n40 / n40 / n40;"><span>even</span></div>
                <div id="n41" style="grid-area: n41 / n41 / n41 / n41;"><span>odd</span></div>
                <div id="n31" style="grid-area: n31 / n31 / n31 / n31;"><span>19-36</span></div>
                <div id="n01" style="grid-area: n01 / n01 / n01 / n01;">
                  <svg viewBox="0 0 100 50"><polygon points="50,0 100,25 50,50 0,25"></polygon></svg>
                </div>
                <div id="n00" style="grid-area: n00 / n00 / n00 / n00;">
                  <svg viewBox="0 0 100 50"><polygon points="50,0 100,25 50,50 0,25"></polygon></svg>
              </div>
              </div>
              <div class="streets"><div id="st0" style="grid-area: st0 / st0 / st0 / st0;"></div>
                <div id="st1" style="grid-area: st1 / st1 / st1 / st1;"></div>
                <div id="st2" style="grid-area: st2 / st2 / st2 / st2;"></div>
                <div id="st3" style="grid-area: st3 / st3 / st3 / st3;"></div>
                <div id="st4" style="grid-area: st4 / st4 / st4 / st4;"></div>
                <div id="st5" style="grid-area: st5 / st5 / st5 / st5;"></div>
                <div id="st6" style="grid-area: st6 / st6 / st6 / st6;"></div>
                <div id="st7" style="grid-area: st7 / st7 / st7 / st7;"></div>
                <div id="st8" style="grid-area: st8 / st8 / st8 / st8;"></div>
                <div id="st9" style="grid-area: st9 / st9 / st9 / st9;"></div>
                <div id="st10" style="grid-area: st10 / st10 / st10 / st10;"></div>
                <div id="st11" style="grid-area: st11 / st11 / st11 / st11;"></div>
              </div>
              <div class="verts">
                <div id="ve0" style="grid-area: ve0 / ve0 / ve0 / ve0;"></div>
                <div id="ve1" style="grid-area: ve1 / ve1 / ve1 / ve1;"></div>
                <div id="ve2" style="grid-area: ve2 / ve2 / ve2 / ve2;"></div>
                <div id="ve3" style="grid-area: ve3 / ve3 / ve3 / ve3;"></div>
                <div id="ve4" style="grid-area: ve4 / ve4 / ve4 / ve4;"></div>
                <div id="ve5" style="grid-area: ve5 / ve5 / ve5 / ve5;"></div>
                <div id="ve6" style="grid-area: ve6 / ve6 / ve6 / ve6;"></div>
                <div id="ve7" style="grid-area: ve7 / ve7 / ve7 / ve7;"></div>
                <div id="ve8" style="grid-area: ve8 / ve8 / ve8 / ve8;"></div>
                <div id="ve9" style="grid-area: ve9 / ve9 / ve9 / ve9;"></div>
                <div id="ve10" style="grid-area: ve10 / ve10 / ve10 / ve10;"></div>
                <div id="ve11" style="grid-area: ve11 / ve11 / ve11 / ve11;"></div>
                <div id="ve12" style="grid-area: ve12 / ve12 / ve12 / ve12;"></div>
                <div id="ve13" style="grid-area: ve13 / ve13 / ve13 / ve13;"></div>
                <div id="ve14" style="grid-area: ve14 / ve14 / ve14 / ve14;"></div>
                <div id="ve15" style="grid-area: ve15 / ve15 / ve15 / ve15;"></div>
                <div id="ve16" style="grid-area: ve16 / ve16 / ve16 / ve16;"></div>
                <div id="ve17" style="grid-area: ve17 / ve17 / ve17 / ve17;"></div>
                <div id="ve18" style="grid-area: ve18 / ve18 / ve18 / ve18;"></div>
                <div id="ve19" style="grid-area: ve19 / ve19 / ve19 / ve19;"></div>
                <div id="ve20" style="grid-area: ve20 / ve20 / ve20 / ve20;"></div>
                <div id="ve21" style="grid-area: ve21 / ve21 / ve21 / ve21;"></div>
                <div id="ve22" style="grid-area: ve22 / ve22 / ve22 / ve22;"></div>
                <div id="ve23" style="grid-area: ve23 / ve23 / ve23 / ve23;"></div>
              </div>
              <div class="horiz">
                <div id="ho0" style="grid-area: ho0 / ho0 / ho0 / ho0;"></div>
                <div id="ho1" style="grid-area: ho1 / ho1 / ho1 / ho1;"></div>
                <div id="ho2" style="grid-area: ho2 / ho2 / ho2 / ho2;"></div>
                <div id="ho3" style="grid-area: ho3 / ho3 / ho3 / ho3;"></div>
                <div id="ho4" style="grid-area: ho4 / ho4 / ho4 / ho4;"></div>
                <div id="ho5" style="grid-area: ho5 / ho5 / ho5 / ho5;"></div>
                <div id="ho6" style="grid-area: ho6 / ho6 / ho6 / ho6;"></div>
                <div id="ho7" style="grid-area: ho7 / ho7 / ho7 / ho7;"></div>
                <div id="ho8" style="grid-area: ho8 / ho8 / ho8 / ho8;"></div>
                <div id="ho9" style="grid-area: ho9 / ho9 / ho9 / ho9;"></div>
                <div id="ho10" style="grid-area: ho10 / ho10 / ho10 / ho10;"></div>
                <div id="ho11" style="grid-area: ho11 / ho11 / ho11 / ho11;"></div>
                <div id="ho12" style="grid-area: ho12 / ho12 / ho12 / ho12;"></div>
                <div id="ho13" style="grid-area: ho13 / ho13 / ho13 / ho13;"></div>
                <div id="ho14" style="grid-area: ho14 / ho14 / ho14 / ho14;"></div>
                <div id="ho15" style="grid-area: ho15 / ho15 / ho15 / ho15;"></div>
                <div id="ho16" style="grid-area: ho16 / ho16 / ho16 / ho16;"></div>
                <div id="ho17" style="grid-area: ho17 / ho17 / ho17 / ho17;"></div>
                <div id="ho18" style="grid-area: ho18 / ho18 / ho18 / ho18;"></div>
                <div id="ho19" style="grid-area: ho19 / ho19 / ho19 / ho19;"></div>
                <div id="ho20" style="grid-area: ho20 / ho20 / ho20 / ho20;"></div>
                <div id="ho21" style="grid-area: ho21 / ho21 / ho21 / ho21;"></div>
                <div id="ho22" style="grid-area: ho22 / ho22 / ho22 / ho22;"></div>
                <div id="ho23" style="grid-area: ho23 / ho23 / ho23 / ho23;"></div>
                <div id="ho24" style="grid-area: ho24 / ho24 / ho24 / ho24;"></div>
                <div id="ho25" style="grid-area: ho25 / ho25 / ho25 / ho25;"></div>
                <div id="ho26" style="grid-area: ho26 / ho26 / ho26 / ho26;"></div>
                <div id="ho27" style="grid-area: ho27 / ho27 / ho27 / ho27;"></div>
                <div id="ho28" style="grid-area: ho28 / ho28 / ho28 / ho28;"></div>
                <div id="ho29" style="grid-area: ho29 / ho29 / ho29 / ho29;"></div>
                <div id="ho30" style="grid-area: ho30 / ho30 / ho30 / ho30;"></div>
                <div id="ho31" style="grid-area: ho31 / ho31 / ho31 / ho31;"></div>
                <div id="ho32" style="grid-area: ho32 / ho32 / ho32 / ho32;"></div>
                <div id="ho33" style="grid-area: ho33 / ho33 / ho33 / ho33;"></div>
                <div id="ho34" style="grid-area: ho34 / ho34 / ho34 / ho34;"></div>
                <div id="ho35" style="grid-area: ho35 / ho35 / ho35 / ho35;"></div>
              </div>
              <div class="squares">
                <div id="sq0" style="grid-area: sq0 / sq0 / sq0 / sq0;"></div>
                <div id="sq1" style="grid-area: sq1 / sq1 / sq1 / sq1;"></div>
                <div id="sq2" style="grid-area: sq2 / sq2 / sq2 / sq2;"></div>
                <div id="sq3" style="grid-area: sq3 / sq3 / sq3 / sq3;"></div>
                <div id="sq4" style="grid-area: sq4 / sq4 / sq4 / sq4;"></div>
                <div id="sq5" style="grid-area: sq5 / sq5 / sq5 / sq5;"></div>
                <div id="sq6" style="grid-area: sq6 / sq6 / sq6 / sq6;"></div>
                <div id="sq7" style="grid-area: sq7 / sq7 / sq7 / sq7;"></div>
                <div id="sq8" style="grid-area: sq8 / sq8 / sq8 / sq8;"></div>
                <div id="sq9" style="grid-area: sq9 / sq9 / sq9 / sq9;"></div>
                <div id="sq10" style="grid-area: sq10 / sq10 / sq10 / sq10;"></div>
                <div id="sq11" style="grid-area: sq11 / sq11 / sq11 / sq11;"></div>
                <div id="sq12" style="grid-area: sq12 / sq12 / sq12 / sq12;"></div>
                <div id="sq13" style="grid-area: sq13 / sq13 / sq13 / sq13;"></div>
                <div id="sq14" style="grid-area: sq14 / sq14 / sq14 / sq14;"></div>
                <div id="sq15" style="grid-area: sq15 / sq15 / sq15 / sq15;"></div>
                <div id="sq16" style="grid-area: sq16 / sq16 / sq16 / sq16;"></div>
                <div id="sq17" style="grid-area: sq17 / sq17 / sq17 / sq17;"></div>
                <div id="sq18" style="grid-area: sq18 / sq18 / sq18 / sq18;"></div>
                <div id="sq19" style="grid-area: sq19 / sq19 / sq19 / sq19;"></div>
                <div id="sq20" style="grid-area: sq20 / sq20 / sq20 / sq20;"></div>
                <div id="sq21" style="grid-area: sq21 / sq21 / sq21 / sq21;"></div>
                <div id="sq22" style="grid-area: sq22 / sq22 / sq22 / sq22;"></div>
                <div id="sq23" style="grid-area: sq23 / sq23 / sq23 / sq23;"></div>
              </div>
            </div>
          </div>
          <div class="bet-select">
            <div class="bet-option" :class="amount_select == 1 ? 'active':''" @click="select_bet(1)">
              <div class="chip">
                <svg viewBox="0 0 60 60"><circle opacity=".3" fill="#231F20" cx="30" cy="31" r="27.5"></circle><circle fill="#357f9c" cx="30" cy="30" r="26.5"></circle><g fill="#FFF"><path d="M26.348 8h-8.877a22.019 22.019 0 00-7.774 9.637h4.17a20.236 20.236 0 00-2.623 11.082 21.962 21.962 0 017.368-11.082 21.878 21.878 0 016.079-3.456h-8.356A20.336 20.336 0 0126.348 8zM52 26.348v-8.88a22.007 22.007 0 00-9.64-7.772v4.167a20.221 20.221 0 00-11.079-2.618 21.931 21.931 0 0111.079 7.366 21.919 21.919 0 013.459 6.077l.001-8.354A20.365 20.365 0 0152 26.348zM33.652 52h8.877a22.03 22.03 0 007.774-9.637h-4.17a20.237 20.237 0 002.623-11.082 21.958 21.958 0 01-7.368 11.082 21.9 21.9 0 01-6.079 3.455h8.356A20.35 20.35 0 0133.652 52zM8 33.652v8.879a22.003 22.003 0 009.64 7.773v-4.168a20.21 20.21 0 0011.079 2.617 21.93 21.93 0 01-11.079-7.365 21.945 21.945 0 01-3.46-6.076v8.354A20.365 20.365 0 018 33.652z"></path></g><text fill="#FEFEFE" x="50%" y="50%" font-size="15" font-weight="500" text-anchor="middle" dy=".33em">1</text></svg>
              </div>
            </div>
            <div class="bet-option" :class="amount_select == 5 ? 'active':''" @click="select_bet(5)">
              <div class="chip">
                <svg viewBox="0 0 60 60"><circle opacity=".3" fill="#231F20" cx="30" cy="31" r="27.5"></circle><circle fill="#aa2f2a" cx="30" cy="30" r="26.5"></circle><g fill="#FFF"><path d="M26.348 8h-8.877a22.019 22.019 0 00-7.774 9.637h4.17a20.236 20.236 0 00-2.623 11.082 21.962 21.962 0 017.368-11.082 21.878 21.878 0 016.079-3.456h-8.356A20.336 20.336 0 0126.348 8zM52 26.348v-8.88a22.007 22.007 0 00-9.64-7.772v4.167a20.221 20.221 0 00-11.079-2.618 21.931 21.931 0 0111.079 7.366 21.919 21.919 0 013.459 6.077l.001-8.354A20.365 20.365 0 0152 26.348zM33.652 52h8.877a22.03 22.03 0 007.774-9.637h-4.17a20.237 20.237 0 002.623-11.082 21.958 21.958 0 01-7.368 11.082 21.9 21.9 0 01-6.079 3.455h8.356A20.35 20.35 0 0133.652 52zM8 33.652v8.879a22.003 22.003 0 009.64 7.773v-4.168a20.21 20.21 0 0011.079 2.617 21.93 21.93 0 01-11.079-7.365 21.945 21.945 0 01-3.46-6.076v8.354A20.365 20.365 0 018 33.652z"></path></g><text fill="#FEFEFE" x="50%" y="50%" font-size="15" font-weight="500" text-anchor="middle" dy=".33em">5</text></svg>
              </div>
            </div>
            <div class="bet-option" :class="amount_select == 10 ? 'active':''" @click="select_bet(10)">
              <div class="chip">
                <svg viewBox="0 0 60 60"><circle opacity=".3" fill="#231F20" cx="30" cy="31" r="27.5"></circle><circle fill="#31ba9c" cx="30" cy="30" r="26.5"></circle><g fill="#FFF"><path d="M26.348 8h-8.877a22.019 22.019 0 00-7.774 9.637h4.17a20.236 20.236 0 00-2.623 11.082 21.962 21.962 0 017.368-11.082 21.878 21.878 0 016.079-3.456h-8.356A20.336 20.336 0 0126.348 8zM52 26.348v-8.88a22.007 22.007 0 00-9.64-7.772v4.167a20.221 20.221 0 00-11.079-2.618 21.931 21.931 0 0111.079 7.366 21.919 21.919 0 013.459 6.077l.001-8.354A20.365 20.365 0 0152 26.348zM33.652 52h8.877a22.03 22.03 0 007.774-9.637h-4.17a20.237 20.237 0 002.623-11.082 21.958 21.958 0 01-7.368 11.082 21.9 21.9 0 01-6.079 3.455h8.356A20.35 20.35 0 0133.652 52zM8 33.652v8.879a22.003 22.003 0 009.64 7.773v-4.168a20.21 20.21 0 0011.079 2.617 21.93 21.93 0 01-11.079-7.365 21.945 21.945 0 01-3.46-6.076v8.354A20.365 20.365 0 018 33.652z"></path></g><text fill="#FEFEFE" x="50%" y="50%" font-size="15" font-weight="500" text-anchor="middle" dy=".33em">10</text></svg>
              </div>
            </div>
            <div class="bet-option" :class="amount_select == 20 ? 'active':''" @click="select_bet(20)">
              <div class="chip">
                <svg viewBox="0 0 60 60"><circle opacity=".3" fill="#231F20" cx="30" cy="31" r="27.5"></circle><circle fill="#d40f83" cx="30" cy="30" r="26.5"></circle><g fill="#FFF"><path d="M26.348 8h-8.877a22.019 22.019 0 00-7.774 9.637h4.17a20.236 20.236 0 00-2.623 11.082 21.962 21.962 0 017.368-11.082 21.878 21.878 0 016.079-3.456h-8.356A20.336 20.336 0 0126.348 8zM52 26.348v-8.88a22.007 22.007 0 00-9.64-7.772v4.167a20.221 20.221 0 00-11.079-2.618 21.931 21.931 0 0111.079 7.366 21.919 21.919 0 013.459 6.077l.001-8.354A20.365 20.365 0 0152 26.348zM33.652 52h8.877a22.03 22.03 0 007.774-9.637h-4.17a20.237 20.237 0 002.623-11.082 21.958 21.958 0 01-7.368 11.082 21.9 21.9 0 01-6.079 3.455h8.356A20.35 20.35 0 0133.652 52zM8 33.652v8.879a22.003 22.003 0 009.64 7.773v-4.168a20.21 20.21 0 0011.079 2.617 21.93 21.93 0 01-11.079-7.365 21.945 21.945 0 01-3.46-6.076v8.354A20.365 20.365 0 018 33.652z"></path></g><text fill="#FEFEFE" x="50%" y="50%" font-size="15" font-weight="500" text-anchor="middle" dy=".33em">20</text></svg>
              </div>
            </div>
            <div class="bet-option" :class="amount_select == 50 ? 'active':''" @click="select_bet(50)">
              <div class="chip">
                <svg viewBox="0 0 60 60"><circle opacity=".3" fill="#231F20" cx="30" cy="31" r="27.5"></circle><circle fill="#412ebc" cx="30" cy="30" r="26.5"></circle><g fill="#FFF"><path d="M26.348 8h-8.877a22.019 22.019 0 00-7.774 9.637h4.17a20.236 20.236 0 00-2.623 11.082 21.962 21.962 0 017.368-11.082 21.878 21.878 0 016.079-3.456h-8.356A20.336 20.336 0 0126.348 8zM52 26.348v-8.88a22.007 22.007 0 00-9.64-7.772v4.167a20.221 20.221 0 00-11.079-2.618 21.931 21.931 0 0111.079 7.366 21.919 21.919 0 013.459 6.077l.001-8.354A20.365 20.365 0 0152 26.348zM33.652 52h8.877a22.03 22.03 0 007.774-9.637h-4.17a20.237 20.237 0 002.623-11.082 21.958 21.958 0 01-7.368 11.082 21.9 21.9 0 01-6.079 3.455h8.356A20.35 20.35 0 0133.652 52zM8 33.652v8.879a22.003 22.003 0 009.64 7.773v-4.168a20.21 20.21 0 0011.079 2.617 21.93 21.93 0 01-11.079-7.365 21.945 21.945 0 01-3.46-6.076v8.354A20.365 20.365 0 018 33.652z"></path></g><text fill="#FEFEFE" x="50%" y="50%" font-size="15" font-weight="500" text-anchor="middle" dy=".33em">50</text></svg>
              </div>
            </div>
            <div class="bet-option" :class="amount_select == 100 ? 'active':''" @click="select_bet(100)">
              <div class="chip">
                <svg viewBox="0 0 60 60"><circle opacity=".3" fill="#231F20" cx="30" cy="31" r="27.5"></circle><circle fill="#2d3b2c" cx="30" cy="30" r="26.5"></circle><g fill="#FFF"><path d="M26.348 8h-8.877a22.019 22.019 0 00-7.774 9.637h4.17a20.236 20.236 0 00-2.623 11.082 21.962 21.962 0 017.368-11.082 21.878 21.878 0 016.079-3.456h-8.356A20.336 20.336 0 0126.348 8zM52 26.348v-8.88a22.007 22.007 0 00-9.64-7.772v4.167a20.221 20.221 0 00-11.079-2.618 21.931 21.931 0 0111.079 7.366 21.919 21.919 0 013.459 6.077l.001-8.354A20.365 20.365 0 0152 26.348zM33.652 52h8.877a22.03 22.03 0 007.774-9.637h-4.17a20.237 20.237 0 002.623-11.082 21.958 21.958 0 01-7.368 11.082 21.9 21.9 0 01-6.079 3.455h8.356A20.35 20.35 0 0133.652 52zM8 33.652v8.879a22.003 22.003 0 009.64 7.773v-4.168a20.21 20.21 0 0011.079 2.617 21.93 21.93 0 01-11.079-7.365 21.945 21.945 0 01-3.46-6.076v8.354A20.365 20.365 0 018 33.652z"></path></g><text fill="#FEFEFE" x="50%" y="50%" font-size="15" font-weight="500" text-anchor="middle" dy=".33em">100</text></svg>
              </div>
            </div>
            <div class="bet-option" :class="amount_select == 500 ? 'active':''" @click="select_bet(500)">
              <div class="chip">
                <svg viewBox="0 0 60 60"><circle opacity=".3" fill="#231F20" cx="30" cy="31" r="27.5"></circle><circle fill="#993cae" cx="30" cy="30" r="26.5"></circle><g fill="#FFF"><path d="M26.348 8h-8.877a22.019 22.019 0 00-7.774 9.637h4.17a20.236 20.236 0 00-2.623 11.082 21.962 21.962 0 017.368-11.082 21.878 21.878 0 016.079-3.456h-8.356A20.336 20.336 0 0126.348 8zM52 26.348v-8.88a22.007 22.007 0 00-9.64-7.772v4.167a20.221 20.221 0 00-11.079-2.618 21.931 21.931 0 0111.079 7.366 21.919 21.919 0 013.459 6.077l.001-8.354A20.365 20.365 0 0152 26.348zM33.652 52h8.877a22.03 22.03 0 007.774-9.637h-4.17a20.237 20.237 0 002.623-11.082 21.958 21.958 0 01-7.368 11.082 21.9 21.9 0 01-6.079 3.455h8.356A20.35 20.35 0 0133.652 52zM8 33.652v8.879a22.003 22.003 0 009.64 7.773v-4.168a20.21 20.21 0 0011.079 2.617 21.93 21.93 0 01-11.079-7.365 21.945 21.945 0 01-3.46-6.076v8.354A20.365 20.365 0 018 33.652z"></path></g><text fill="#FEFEFE" x="50%" y="50%" font-size="15" font-weight="500" text-anchor="middle" dy=".33em">500</text></svg>
              </div>
            </div>
            <div class="bet-option" :class="amount_select == 1000 ? 'active':''" @click="select_bet(1000)">
              <div class="chip">
                <svg viewBox="0 0 60 60"><circle opacity=".3" fill="#231F20" cx="30" cy="31" r="27.5"></circle><circle fill="#d45c20" cx="30" cy="30" r="26.5"></circle><g fill="#FFF"><path d="M26.348 8h-8.877a22.019 22.019 0 00-7.774 9.637h4.17a20.236 20.236 0 00-2.623 11.082 21.962 21.962 0 017.368-11.082 21.878 21.878 0 016.079-3.456h-8.356A20.336 20.336 0 0126.348 8zM52 26.348v-8.88a22.007 22.007 0 00-9.64-7.772v4.167a20.221 20.221 0 00-11.079-2.618 21.931 21.931 0 0111.079 7.366 21.919 21.919 0 013.459 6.077l.001-8.354A20.365 20.365 0 0152 26.348zM33.652 52h8.877a22.03 22.03 0 007.774-9.637h-4.17a20.237 20.237 0 002.623-11.082 21.958 21.958 0 01-7.368 11.082 21.9 21.9 0 01-6.079 3.455h8.356A20.35 20.35 0 0133.652 52zM8 33.652v8.879a22.003 22.003 0 009.64 7.773v-4.168a20.21 20.21 0 0011.079 2.617 21.93 21.93 0 01-11.079-7.365 21.945 21.945 0 01-3.46-6.076v8.354A20.365 20.365 0 018 33.652z"></path></g><text fill="#FEFEFE" x="50%" y="50%" font-size="15" font-weight="500" text-anchor="middle" dy=".33em">1K</text></svg>
              </div>
            </div>
            <div class="bet-option" :class="amount_select == 2000 ? 'active':''" @click="select_bet(2000)">
              <div class="chip">
                <svg viewBox="0 0 60 60"><circle opacity=".3" fill="#231F20" cx="30" cy="31" r="27.5"></circle><circle fill="#cec440" cx="30" cy="30" r="26.5"></circle><g fill="#FFF"><path d="M26.348 8h-8.877a22.019 22.019 0 00-7.774 9.637h4.17a20.236 20.236 0 00-2.623 11.082 21.962 21.962 0 017.368-11.082 21.878 21.878 0 016.079-3.456h-8.356A20.336 20.336 0 0126.348 8zM52 26.348v-8.88a22.007 22.007 0 00-9.64-7.772v4.167a20.221 20.221 0 00-11.079-2.618 21.931 21.931 0 0111.079 7.366 21.919 21.919 0 013.459 6.077l.001-8.354A20.365 20.365 0 0152 26.348zM33.652 52h8.877a22.03 22.03 0 007.774-9.637h-4.17a20.237 20.237 0 002.623-11.082 21.958 21.958 0 01-7.368 11.082 21.9 21.9 0 01-6.079 3.455h8.356A20.35 20.35 0 0133.652 52zM8 33.652v8.879a22.003 22.003 0 009.64 7.773v-4.168a20.21 20.21 0 0011.079 2.617 21.93 21.93 0 01-11.079-7.365 21.945 21.945 0 01-3.46-6.076v8.354A20.365 20.365 0 018 33.652z"></path></g><text fill="#FEFEFE" x="50%" y="50%" font-size="15" font-weight="500" text-anchor="middle" dy=".33em">2K</text></svg>
              </div>
            </div>
            <div class="bet-option" :class="amount_select == 5000 ? 'active':''" @click="select_bet(5000)">
              <div class="chip">
                <svg viewBox="0 0 60 60"><circle opacity=".3" fill="#231F20" cx="30" cy="31" r="27.5"></circle><circle fill="#2c9e24" cx="30" cy="30" r="26.5"></circle><g fill="#FFF"><path d="M26.348 8h-8.877a22.019 22.019 0 00-7.774 9.637h4.17a20.236 20.236 0 00-2.623 11.082 21.962 21.962 0 017.368-11.082 21.878 21.878 0 016.079-3.456h-8.356A20.336 20.336 0 0126.348 8zM52 26.348v-8.88a22.007 22.007 0 00-9.64-7.772v4.167a20.221 20.221 0 00-11.079-2.618 21.931 21.931 0 0111.079 7.366 21.919 21.919 0 013.459 6.077l.001-8.354A20.365 20.365 0 0152 26.348zM33.652 52h8.877a22.03 22.03 0 007.774-9.637h-4.17a20.237 20.237 0 002.623-11.082 21.958 21.958 0 01-7.368 11.082 21.9 21.9 0 01-6.079 3.455h8.356A20.35 20.35 0 0133.652 52zM8 33.652v8.879a22.003 22.003 0 009.64 7.773v-4.168a20.21 20.21 0 0011.079 2.617 21.93 21.93 0 01-11.079-7.365 21.945 21.945 0 01-3.46-6.076v8.354A20.365 20.365 0 018 33.652z"></path></g><text fill="#FEFEFE" x="50%" y="50%" font-size="15" font-weight="500" text-anchor="middle" dy=".33em">5K</text></svg>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div class="line" style="margin-top:40px;">
        <div class="left">
          <div class="info">
            <div class="balance">BALANCE<p><span class="padded">{{balance_zeros}}</span><span>{{balance_amount}}</span></p></div>
            <div class="bet">BET<p><span class="padded">{{bet_zeros}}</span><span>{{bet_amount}}</span></p></div>
          </div>
        </div>
        <div class="right">
          <fieldset class="actions" ref="actions">
            <button aria-label="Clear Board" @click="rebet()">Rebet</button>
            <button aria-label="Clear Board" @click="clear()">Clear</button>
            <button aria-label="Undo" @click="undo()">undo</button>
            <button aria-label="Clear Board" @click="bet_submit()">BET</button>
          </fieldset>
        </div>
      </div>
    </div>
    <!--menu-->
    <div class="menu" @click.stop>
      <div class="menu-item" @click="toggle_deposit()">DEPOSIT</div>
      <div class="menu-item" @click="toggle_withdraw()">WITHDRAW</div>
      <div class="menu-item" @click="toggle_history()">{{history_visible ? 'HIDE HISTORY':'SHOW HISTORY'}}</div>
    </div>   
    <!--mask-->
    <div class="mask" v-if="deposit_visible || withdraw_visible || history_visible" @click.stop>

      <div class="mask-content" v-if="deposit_visible">
        <input v-model="deposit_text"/>
        <div class="button-box">
          <div class="button" @click="deposit_visible=false">cancel</div>
          <div class="button" @click="toggle_deposit()">confirm</div>
        </div>
      </div>
      <div class="mask-content mask-content-withdraw" v-if="withdraw_visible">
        <input v-model="withdraw_text"/>
        <div class="tip">1 chip = 0.01 NEAR</div>
        <div class="button-box">
          <div class="button" @click="withdraw_visible=false">cancel</div>
          <div class="button" @click="toggle_withdraw()">confirm</div>
        </div>
      </div>
      <div class="mask-content mask-content-history" v-if="history_visible">
        <img class="close-btn" @click="history_visible=false" src="@/assets/close.png">
        <div class="tap-box">
          <div :class="{'active':current_type=='number'}" @click="current_type='number'">number</div>
          <div :class="{'active':current_type=='bet'}" @click="current_type='bet'">bet</div>
        </div>
        <div class="list" v-if="current_type=='number'">
          <div class="history-wrapper" v-for="(number, i) in history_numbers" :key="i">
            <div class="history" style="margin-right: 30px">
              ROUND: {{number.round_index}}
            </div>
            <div class="history" style="margin-right: 30px">
              NUMBER: {{number.win_number}}
            </div>
          </div>
        </div>
        <div class="list" v-if="current_type=='bet'">
          <div class="history-wrapper" v-for="(number, i) in history_bets" :key="i">
            <div class="history" style="margin-right: 30px">
              ROUND: {{number.round_index}}
            </div>
            <div class="history" style="margin-right: 30px">
              NUMBER: {{number.win_number}}
            </div>
            <div class="history" style="margin-right: 30px">
              BET: {{number.bet_chips}}
            </div>
            <div class="history" style="margin-right: 30px">
              WON: {{Math.floor(Number(contract.from_yocto(number.win_chips)) * 100)}}
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
  import Contract from '../contract'
  import wampApi from "../wamp/api.js";
  const CHIP_VALUES = [1, 5, 10, 20, 50, 100, 500, 1000, 2000, 5000];
  const COLORS = [
    "#357f9c",
    "#aa2f2a",
    "#31ba9c",
    "#d40f83",
    "#412ebc",
    "#2d3b2c",
    "#993cae",
    "#d45c20",
    "#cec440",
    "#2c9e24"
  ];
  const DISPLAY_VALUES = {
    1000: "1K",
    2000: "2K",
    5000: "5K"
  };
  const createMutateStyle = (top = 0, left = 0, deg = 0) => {
    top = (top * 15).toFixed() + "%";
    left = (left * 15).toFixed() + "%";
    // top = "0%";
    // left = "0%";
    deg = (deg * 50).toFixed();
    return {
      top,
      left,
      transform: `rotate(${deg}deg)`
    };
  };
  function Chip(height, width, value, style) {
    const color = COLORS[CHIP_VALUES.indexOf(value)] || "black";
    const text = DISPLAY_VALUES[value] || value;
    return String.prototype.concat.call(
      "<div className='chip' style='" + style + "'>",
        "<svg height=" + height + " width=" + width + " viewBox='0 0 60 60'>",
          "<circle opacity='.3' fill='#231F20' cx='30' cy='31' r='27.5' />",
          "<circle fill=" + color +" cx='30' cy='30' r='26.5' />",
          "<g fill='#FFF'>",
            "<path d='M26.348 8h-8.877a22.019 22.019 0 00-7.774 9.637h4.17a20.236 20.236 0 00-2.623 11.082 21.962 21.962 0 017.368-11.082 21.878 21.878 0 016.079-3.456h-8.356A20.336 20.336 0 0126.348 8zM52 26.348v-8.88a22.007 22.007 0 00-9.64-7.772v4.167a20.221 20.221 0 00-11.079-2.618 21.931 21.931 0 0111.079 7.366 21.919 21.919 0 013.459 6.077l.001-8.354A20.365 20.365 0 0152 26.348zM33.652 52h8.877a22.03 22.03 0 007.774-9.637h-4.17a20.237 20.237 0 002.623-11.082 21.958 21.958 0 01-7.368 11.082 21.9 21.9 0 01-6.079 3.455h8.356A20.35 20.35 0 0133.652 52zM8 33.652v8.879a22.003 22.003 0 009.64 7.773v-4.168a20.21 20.21 0 0011.079 2.617 21.93 21.93 0 01-11.079-7.365 21.945 21.945 0 01-3.46-6.076v8.354A20.365 20.365 0 018 33.652z' />",
          "</g>",
          "<text fill='#FEFEFE' x='50%' y='50%' fontSize='15' fontWeight='500' textAnchor='middle' dy='.33em' style='text-anchor: middle;'>",
            text,
          "</text>",
        "</svg>",
      "<div>"
    )
  }
  function check_win(number, b) {
      let won = false;
      if (number == 0) {
          won = b.bet_type == 5 && b.number == 0;                   /* bet on 0 */
      } else {
      if (b.bet_type == 5) { 
          won = b.number == number;                              /* bet on number */
      } else if (b.bet_type == 4) {
          if (b.number == 0) { won = number % 2 == 0; }              /* bet on even */
          if (b.number == 1) { won = number % 2 == 1; }              /* bet on odd */
      } else if (b.bet_type == 3) {            
          if (b.number == 0) { won = number <= 18; }                /* bet on low 18s */
          if (b.number == 1) { won = number >= 19; }                 /* bet on high 18s */
      } else if (b.bet_type == 2) {                               
          if (b.number == 0) { won = number <= 12; }                 /* bet on 1st dozen */
          if (b.number == 1) { won = number > 12 && number <= 24; }  /* bet on 2nd dozen */
          if (b.number == 2) { won = number > 24; }                  /* bet on 3rd dozen */
      } else if (b.bet_type == 1) {               
          if (b.number == 0) { won = number % 3 == 1; }              /* bet on left column */
          if (b.number == 1) { won = number % 3 == 2; }              /* bet on middle column */
          if (b.number == 2) { won = number % 3 == 0; }              /* bet on right column */
      } else if (b.bet_type == 0) {
          if (b.number == 0) {                                     /* bet on black */
            if (number <= 10 || number >= 20 && number <= 28) {
                won = number % 2 == 0;
            } else {
                won = number % 2 == 1;
            }
          } else {                                                 /* bet on red */
            if (number <= 10 || number >= 20 && number <= 28) {
                won = number % 2 == 1;
            } else {
                won = number % 2 == 0;
            }
          }
      }
      }
      return won
  }

  export default {
    data() {
      return {
        login: false,
        accountId: '',
        contract: {},
        bets: [],
        last_bets: [],
        amount_select: 10,
        balance_zeros: "000000",
        balance_amount: 0,
        balance: 0,
        bet_zeros: "000000",
        bet_amount: 0,
        bet_val: 0,
        deposit_text: 100,
        withdraw_text: 100,
        deposit_visible: false,
        withdraw_visible: false,
        spinning: false,
        already_bet: false,
        win_number: 0,
        ballSpinCounter: 0,
        lastPosition: 0,
        interval: {},
        time_interval: {},
        round_index: 0,
        remain_time: 0,
        next_round_block_index: 0,
        current_block_index: 0,
        waiting: false,
        bet_disabled: false,
        history_visible: false,
        history_numbers: [],
        history_bets: [],
        current_type:'number',
        round_status:{},
        intervalHandler: 0,
        block_height: 0,
        show_block_height: 0
      }
    },

    async mounted() {
      this.contract = new Contract()
      await this.contract.init()
      
      this.init()
      await this.initLogin()
      this.initBet()
      this.update()
      this.deal_href()
      this.run()
      // this.wait_for_spin()
    },

    async destroyed() {
      clearInterval(this.intervalHandler)
    },

    methods: {
      init() {
        for (let i = 0; i <= 36; i ++) {
          let div = document.getElementById('n5' + i)
          div.onclick = () => { this.bet(i, 5, div) }
        }
        for (let i = 0; i <= 2; i ++) {
          let div = document.getElementById('n2' + i)
          div.onclick = () => { this.bet(i, 2, div) }
        }
        for (let i = 0; i <= 2; i ++) {
          let div = document.getElementById('n1' + i)
          div.onclick = () => { this.bet(i, 1, div) }
        }
        document.getElementById('n30').onclick = () => { this.bet(0, 3, document.getElementById('n30')) }
        document.getElementById('n40').onclick = () => { this.bet(0, 4, document.getElementById('n40')) }
        document.getElementById('n41').onclick = () => { this.bet(1, 4, document.getElementById('n41')) }
        document.getElementById('n31').onclick = () => { this.bet(1, 3, document.getElementById('n31')) }
        document.getElementById('n01').onclick = () => { this.bet(1, 0, document.getElementById('n01')) }
        document.getElementById('n00').onclick = () => { this.bet(0, 0, document.getElementById('n00')) }
      },

      async initLogin() {
        let login = localStorage.getItem("login")
        let accountId = await this.contract.is_signed_in()
        this.login = login || accountId
        if (this.login) {
          localStorage.setItem("login", true)
        } else {
          localStorage.removeItem("login")
        }
      },

      to_stake() {
        this.$router.push('stake')
      },

      initBet() {
        let str = localStorage.getItem("bets")
        let amount_select = Number(localStorage.getItem("amount_select"))
        if (amount_select) {
          this.amount_select = amount_select
        }
        if (!str) {
          return
        }
        let bets = str.split(",")
        for (let i = 0; i < bets.length; i++) {
          let bet_type = Number(bets[i].substring(0, 1))
          let index = bets[i].indexOf("c")
          let number = Number(bets[i].substring(1, index))
          let node = document.getElementById("n" + bet_type + number)
          let chips = Number(bets[i].substring(index + 1))
          this.bet(number, bet_type, node, chips)
        }
      },


      async handler(value) {
        if (this.block_height == 0 || this.show_block_height == 0) {
          this.block_height = Number(value.latestBlockHeight)
          this.show_block_height = Number(value.latestBlockHeight)
        }
        if (this.spinning) {
          return
        }
        // let roundStatus = await this.contract.get_round_status()
        if (this.block_height == Number(value.latestBlockHeight)) {
          this.show_block_height += 1
        } else {
          this.show_block_height = Number(value.latestBlockHeight)
        }
        this.block_height = value.latestBlockHeight

        this.remain_time = Math.ceil((this.round_status.next_round_block_index - this.show_block_height) * value.recentBlockProductionSpeed)
        this.remain_time = this.remain_time < 0 ? 0:this.remain_time
        try {
          if (this.round_status.next_round_block_index < Number(this.show_block_height)) {
            // if(!this.is_pre_animation && this.round_status.round_index==this.round_index){
            //   this.animation(this.win_number, true,true)
            // }
            
            this.round_status = await this.contract.get_round_status()
            if (this.round_status.bet_count > 0 && this.waiting == false) {
              this.waiting = true
              this.waiting_spin_wheel()
              this.intervalHandler = setInterval(() => {
                this.waiting_spin_wheel()
              }, 3000)
            }
            
            if (this.round_status.round_index!=this.round_index) {
              this.waiting = false
            }
          }
        } catch (e) {
          console.log(e)
        }
      },

      async run() {
        this.round_status = await this.contract.get_round_status();
        this.round_index = this.round_status.round_index
        this.win_number = this.round_status.last_round_win_number
        this.animation(this.win_number, true)
        wampApi.subscribe("chain-blocks-stats", (value) => {
          this.handler(value)
        })
      },

      deal_href() {
        let hash = this.$route.query.transactionHashes
        if (!hash) {
          let index = location.href.indexOf("?")
          if (index > -1) {
            location.href = location.href.substring(0, index)
          }
          return false
        } 
        let index = location.href.indexOf("?")
        if (index > -1) {
          location.href = location.href.substring(0, index)
        }
        return true
      },

      async update() {
        if (this.login) {
          let account_status = await this.contract.get_account_status()
          // let contract_status = await this.contract.get_contract_status();
          let round_status = await this.contract.get_round_status();
          // console.log(account_status,contract_status)
          // this.status = status
          let balance_str = this.contract.from_yocto(account_status.balance + "00")
          balance_str = balance_str.replace(/,/g, "");
          this.editBalance(balance_str)
          this.win_number = round_status.last_round_win_number
          // this.history_bets = status.user.history_bets.reverse()
          // for (let i = 0; i < this.history_bets.length; i ++) {
          //   let round = this.history_bets[i]
          //   round.bet_chips = 0
          //   for (let j = 0; j < round.bets.length; j ++) {
          //     round.bet_chips += Number(this.contract.from_yocto(round.bets[j].chips)) * 100
          //   }
          // }
        }
      },

      sign_in() {
        this.contract.sign_in()
      },

      sign_out() {
        this.contract.sign_out()
        this.editBalance(0)
        this.login = false
        localStorage.removeItem("login")
      },

      editBalance(balance) {
        this.balance = Math.floor(Number(balance))
        let i = 0
        let original_balance = this.balance_amount
        let now_balance = this.balance
        if (now_balance < 0) {
          now_balance = 0
        }
        let count = 10
        let delta = Math.floor((now_balance - original_balance) / count)
        if (delta < 1 && delta > -1) {
          count = 1
        }
        let ani = () => {
          i += 1
          let amount = this.balance_amount + delta
          this.balance_amount = amount < 0 ? 0:amount
          if (i == count) {
            this.balance_amount = now_balance
          }
          let zero_count = String(this.balance_amount).length
          this.balance_zeros = ""
          for (let i = 0; i < 8 - zero_count; i++) {
            this.balance_zeros += "0"
          }
          if (i < count) {
            setTimeout(ani, 50)
          }
        }
        ani()
      },

      editBet(balance) {
        this.bet_val = Number(balance)
        let original_balance = this.bet_amount
        let now_balance = Number(balance)
        let count = 10
        let delta = Math.floor((now_balance - original_balance) / count)
        if (delta < 1 && delta > -1) {
          count = 1
        }
        let i = 0
        let ani = () => {
          i += 1
          let amount = this.bet_amount + delta
          this.bet_amount = amount < 0 ? 0:amount
          if (i == count) {
            this.bet_amount = now_balance
          }
          let zero_count = String(this.bet_amount).length
          this.bet_zeros = ""
          for (let i = 0; i < 8 - zero_count; i++) {
            this.bet_zeros += "0"
          }
          if (i < count) {
            setTimeout(ani, 50)
          }
        }
        ani()
      },

      updateBet(bet) {
        let css = createMutateStyle(0.5 - Math.random(), 0.5 - Math.random(), 0.5 - Math.random())
        let style = "top: " + css.top + "; left: " + css.left + "; transform: " + css.transform
        let node = document.createElement("div");
        node.className = "placed-chips"
        node.style = "position: absolute;width: 100%;height: 100%" + style
        node.innerHTML = Chip(35, 35, bet.chips, style)
        bet.child = node
        bet.node.appendChild(node)
      },

      bet(number, bet_type, node, chips=this.amount_select) {
        if (this.already_bet) {
          return
        }
        let bet = {
          bet_type: bet_type,
          number: number,
          chips: chips,
          node: node
        }
        let bet_amount = this.bet_val + chips
        this.updateBet(bet)
        this.editBet(bet_amount)
        this.editBalance(this.balance)
        this.bets.push(bet)
        let bet_save = []
        for (let i = 0; i < this.bets.length; i++) {
          bet_save.push(String(this.bets[i].bet_type) + String(this.bets[i].number) + "c" + String(this.bets[i].chips))
        }
        localStorage.setItem("bets", bet_save)
      },

      async bet_submit() {
        if (!this.login) {
          this.sign_in()
          return
        }
        if (this.already_bet) {
          return
        }
        this.already_bet = true
        this.$refs.actions.disabled = true
        this.$refs.board.disabled = true
        try {
          await this.contract.bet(this.bets,this.round_index)
          this.$toast.top('Bet Success');
          // window.alert("Bet Success");
        } catch {
          this.$refs.actions.disabled = false
          this.$refs.board.disabled = false
          this.already_bet = false
          this.$toast.top('Transaction Expired');
          // window.alert("Transaction Expired");
        }
        this.update()
      },

      rebet() {
        if (!this.login) {
          this.sign_in()
          return
        }
        for (let i = 0; i < this.last_bets.length; i++) {
          let bet = this.last_bets[i]
          this.bet(bet.number, bet.bet_type, bet.node, bet.chips)
        }
       
        this.bet_submit()
      },

      undo() {
        if (this.already_bet) {
          return
        }
        if (this.bets.length <= 0) {
          return
        }
        let bet = this.bets[this.bets.length - 1]
        bet.node.removeChild(bet.child)
        this.editBet(this.bet_val - bet.chips)
        this.editBalance(this.balance)
        this.bets.splice(this.bets.length - 1, 1)
        let bet_save = []
        for (let i = 0; i < this.bets.length; i++) {
          bet_save.push(String(this.bets[i].bet_type) + String(this.bets[i].number) + "c" + String(this.bets[i].chips))
        }
        localStorage.setItem("bets", bet_save)
      },

      clear() {
        if (this.already_bet) {
          return
        }
        for (let i = 0; i < this.bets.length; i++) {
          let bet = this.bets[i]
          bet.node.removeChild(bet.child)
        }
        this.bets = []
        this.editBet(0)
        this.editBalance(this.balance)
        localStorage.removeItem("bets")
      },

      clear_spin() {
        this.last_bets = []
        Object.assign(this.last_bets, this.bets)
        this.bets = []
        localStorage.removeItem("bets")
      },

      clear_ui() {
        for (let i = 0; i < this.last_bets.length; i++) {
          let bet = this.last_bets[i]
          bet.node.removeChild(bet.child)
        }
        this.editBet(0)
      },

      select_bet(amount) {
        this.amount_select = amount
        localStorage.setItem("amount_select", this.amount_select)
      },

      show_won(number) {
        let won = []
        won.push(document.getElementById("n5" + number))
        // won.push(document.getElementById("n4" + (number % 2)))
        // let str = ''
        // if (number <= 18) {
        //   str = "n30"
        // } else if (number >= 19) {
        //   str = "n31"
        // }
        // won.push(document.getElementById(str))
        // if (number <= 12) {
        //   str = "n20"
        // } else if (number > 12 && number <= 24) {
        //   str = "n21"
        // } else if (number > 24) {
        //   str = "n22"
        // }
        // won.push(document.getElementById(str))
        // if (number % 3 == 1) {
        //   str = "n10"
        // } else if (number % 3 == 2) {
        //   str = "n11"
        // } else if (number % 3 == 0) {
        //   str = "n12"
        // }
        // won.push(document.getElementById(str))
        // if (number <= 10 || number >= 20 && number <= 28) {
        //   if (number % 2 == 0) {
        //     str = "n00"
        //   } else if (number % 2 == 1) {
        //     str = "n01"
        //   }
        // }
        // won.push(document.getElementById(str))

        for (let i = 0; i < this.last_bets.length; i ++) {
          let bet = this.last_bets[i]
          if (check_win(number, bet)) {
            won.push(bet.node)
          }
        }

        for (let i = 0; i < won.length; i++) {
          let bet = won[i]
          bet.style.backgroundColor = "orange"
          setTimeout(() => {
            bet.style.backgroundColor = ""
          }, 3000)
        }
      },

      async waiting_spin_wheel() {
        let ball = document.getElementById("ball");
        this.ballSpinCounter += 2;
        var totalDegrees = this.lastPosition + this.ballSpinCounter * 360
        ball.style.transitionDuration = "3s"
        ball.style.transitionTimingFunction = "linear"
        ball.style.transform = "rotate(" + totalDegrees + "deg)";

        if (this.waiting == false) {
          clearInterval(this.intervalHandler)
          setTimeout(() => {
            this.win_number = this.round_status.last_round_win_number
            this.spin_wheel()
            this.round_index = this.round_status.round_index
            this.$refs.actions.disabled = false
            this.$refs.board.disabled = false
          }, 3000)
        }
        
  },

      async spin_wheel() {
        if (this.spinning) {
          return
        }
        this.spinning = true
        this.clear_spin()
        setTimeout(() => {
          this.show_won(this.win_number)
        }, 3000)
        setTimeout(() => {
          this.clear_ui()
          this.update()
          this.spinning = false
          this.already_bet = false
        }, 6000)
        // setTimeout(() => {
        //   this.clear_ui()
        //   this.update()
        //   this.spinning = false
        //   this.already_bet = false
        // }, 8000)
        setTimeout(() => {

        })
        setTimeout(() => {
          this.animation(this.win_number)
        }, 10)
        
        
      },

      async deposit(amount) {
        if (!this.login) {
          this.sign_in()
          return
        }
        this.editBalance(amount)
        await this.contract.deposit(amount)
        this.update()
      },

      async withdraw(amount) {
        if (!this.login) {
          this.sign_in()
          return
        }
        this.editBalance(this.balance - amount)
        await this.contract.withdraw(amount)
        this.update()
      },

      toggle_deposit() {
        this.deposit_visible = !this.deposit_visible
        if (this.deposit_visible) {
          this.withdraw_visible = false
        } else {
          this.deposit(Number(this.deposit_text))
        }
      },

      toggle_withdraw() {
        this.withdraw_visible = !this.withdraw_visible
        if (this.withdraw_visible) {
          this.withdraw_text = this.balance_amount
          this.deposit_visible = false
        } else {
          this.withdraw(Number(this.withdraw_text))
        }
      },

      toggle_history() {
        this.history_visible = !this.history_visible
      },

      clear_button() {
        this.deposit_visible = false
        this.withdraw_visible = false
      },

      animation(oneRandomNumber, init=false) {
        /* listening for events from the smart contract */
        this.ballSpinCounter += 1;
        /* get wheel element */
        var ball = document.getElementById("ball");
        ball.style.animationName
        /* reset wheel */
        // wheel.style.transformOrigin = "190px 190px"
        // wheel.style.transitionDuration = "10s"
        // wheel.style.transform = "rotate(" + this.lastPosition + "deg)";
        
        /* numbers in the wheel, ordered clockwise */
        var numbers = [
          0, 32, 15, 19, 4, 21, 2, 25, 17, 34, 6, 27,
          13, 36, 11, 30, 8, 23, 10, 5, 24, 16, 33, 1,
          20, 14, 31, 9, 22, 18, 29, 7, 28, 12, 35, 3, 26
        ];
        /* calculate how much do we need to rotate to have the random number chosen */
        var numberDegree = numbers.indexOf(oneRandomNumber) * 360 / numbers.length;
        /* add some rounds before to look like it's spinning */
        var numRoundsBefore = this.ballSpinCounter;
        /* calculate total degrees we need to rotate */
        var totalDegrees = (numRoundsBefore * 360) + numberDegree;
        /* rotate the wheel */
        if (init) {
          ball.style.transitionDuration = "0s"
        } else {
          ball.style.transitionDuration = "3s"
        }
        ball.style.transitionTimingFunction = "ease-out"
        
        ball.style.transform = "rotate(" + totalDegrees + "deg)";
        
        /* save position to be able to reset the wheel next time */
        this.lastPosition = numberDegree;
        /* show status on bets after wheel stops */
      }

    },
  }
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
  .index {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    min-height: 100vh;
    height: 100%;
    place-items: center;
    background-color: green;
  }

  .header {
    position: fixed;
    right: 40px;
    top: 20px;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .header .logo{
    width:50px;
  }

  .header .nav{
    display: flex;
  }
  .nav-item{
    margin-left:20px;
    display: flex;
    justify-content: center;
    align-items: center;
    font-size:14px;
    color:#FFF;
    font-weight:bold;
    cursor:pointer;
  }

  .index-wrapper {
    width: 100%;
    height: 100%;
    max-width: 1200px;
  }

  .line {
    width: 100%;
    max-width: 1200px;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .wheel {
    position: relative;
    width: 330px;
    height: 330px;
    max-width: 330px;
    max-height: 330px;
  }

  .place-bet {
    position: absolute;
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;
    display: flex;
    align-content: center;
    justify-content: center;
  }

  .wheel-base {
    position: absolute;
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;
    display: flex;
    align-content: center;
    justify-content: center;
  }

  .wheel text {
      text-anchor: middle;
      font-size: 19px;
      fill: hsla(0,0%,100%,.9);
      font-weight: 500;
  }

  .board-wrap {
      border: 0px solid #fff !important;
      font-size: 20px;
  }

  .board-wrap>div {
      cursor: pointer!important;
  }

  .board {
    width: 97vw;
    max-width: 750px;
    height: 34.64286vw;
    max-height: 267.85714px;
    display: grid;
    grid-area: board1;
    grid-template-columns: repeat(14,1fr);
    grid-template-areas:
        "n50 n53 n56 n59 n512 n515 n518 n521 n524 n527 n530 n533 n536 n12"
        "n50 n52 n55 n58 n511 n514 n517 n520 n523 n526 n529 n532 n535 n11"
        "n50 n51 n54 n57 n510 n513 n516 n519 n522 n525 n528 n531 n534 n10"
        ". n20 n20 n20 n20 n21 n21 n21 n21 n22 n22 n22 n22 ."
        ". n30 n30 n40 n40 n01 n01 n00 n00 n41 n41 n31 n31 .";
    color: white;
  }

  .board>div {
      display: grid;
      position: relative;
      place-items: center;
      text-transform: uppercase;
      border-top: 1px solid #fff;
      border-right: 1px solid #fff;
      z-index: 100;
      font-size:18px;
      font-weight:bold;
  }

  .board>#n50, .board>#n30, .board>#n20 {
      border-left: 1px solid #fff;
  }
  .board>#n00, .board>#n40, .board>#n31, .board>#n50, .board>#n41, .board>#n01, .board>#n30, .board>#n10 {
      border-bottom: 1px solid #fff;
  }

  #n51, #n54, #n57, #n510, #n513, #n516, #n519, #n522, #n525, #n528, #n531, #n534 {
      box-shadow: 0 4px 0 rgb(0 0 0 / 20%);
  }

  .board .red {
      background-color: #ec0230;
  }

  .board .black {
      background-color: #2b2627;
  }

  .board #n01 polygon {
      fill: #ec0230;
      stroke: #fff;
  }

  .bet-select {
      grid-area: chips;
      margin: 9px;
      margin-top: 30px;
      display: grid;
      grid-template-columns: repeat(10,minmax(auto,68px));
      grid-column-gap: 4px;
      -webkit-justify-content: center;
      justify-content: center;
      place-items: center;
  }

  .bet-option.active {
      -webkit-transform: translateY(-10px);
      transform: translateY(-10px);
  }

  .bet-option {
      transition: all .3s;
  }

  .bet-option, .chip {
      width: 100%;
  }

  .bet-select .chip {
      cursor: pointer;
  }

  .bet-option, .chip {
      width: 100%;
  }

  .info {
    display: flex;
    justify-content: space-between;
    color: white;
    width: 330px;
  }

  .info>div {
    min-width: 75px;
    font-size:16px;
  }

  .info .padded {
    opacity: .12;
  }

  p {
    margin: 0;
    padding: 0;
    font-weight: 500;
    font-size: 20px;
  }

  .actions {
      width:750px;
      display: flex;
      margin:0;
      padding:0;
      justify-content: center;
  }

  .actions button {
      opacity: 1;
      transition: opacity .15s;
      outline: none;
      outline-offset: 0;
      font-size: 16px;
      border:1px solid #FFF;
      width:100px;
      margin-left:20px;
      border-radius:5px;
  }

  button {
      font-family: "Oswald",sans-serif;
      border: none;
      outline: 1px solid #fff;
      outline-offset: -6px;
      padding: 10px 21px;
      margin: 0;
      text-decoration: none;
      text-transform: uppercase;
      background: transparent;
      color: #fff;
      font-weight: 500;
      font-size: 24px;
      cursor: pointer;
      text-align: center;
      transition: background .25s ease-in-out,-webkit-transform .15s ease;
      transition: background .25s ease-in-out,transform .15s ease;
      transition: background .25s ease-in-out,transform .15s ease,-webkit-transform .15s ease;
  }

  button svg {
      fill: #fff;
  }

  button.spin {
      outline: none;
      padding: 0;
      font-weight: 500;
      width: 60%;
      font-size: 18px;
  }

  .board-wrap>div>div:hover{
    background-color:orange
  }

  button:hover {
    background: rgba(0,0,0,0.2)
  }

  .round {
    border-radius: 50%;
  }

  .placed-chips {
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
  }

  .placed-chips .chip {
      position: relative;
      width: 5.5vw;
      max-width: 30px;
      height: 5.5vw;
      max-height: 30px;
  }

  .chip text {
      text-anchor: middle;
      font-size: 19px;
      fill: hsla(0,0%,100%,.9);
      font-weight: 500;
  }

  .wallet-wrapper {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    margin-top: 90px;
  }

  .wallet-unit {
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .mask-content {
    width: 300px;
    border: none;
    outline: 1px solid #fff;
    padding: 30px;
    margin: 0;
    text-decoration: none;
    text-transform: uppercase;
    background: #FFF;
    border-radius:10px;
  }

  .mask-content input {
      outline-color: invert;
      outline-style: none;
      outline-width: 0px;
      border: 1px solid #333;
      padding:0 10px;
      text-shadow: none;
      -webkit-appearance: none;
      outline-color: transparent;
      box-shadow: none;
      background-color: transparent;
      text-align: right;
      width: 100%;
      height:36px;
      line-height:36px;
      font-family: "Oswald",sans-serif;
      color: #333;
      font-weight: 500;
      font-size: 16px;
      box-sizing:border-box;
      border-radius:5px;
  }

  .mask-content-withdraw .tip{
    font-size:12px;
    text-align:right;
    margin-top:10px;
    color:#999;
  }

  .mask-content .button-box{
    display:flex;
    margin-top:30px;
    justify-content: right;
    
  }

  .mask-content .button{
    margin-left:20px;
    font-size:14px;
    color:#333;
    font-weight:bold;
    cursor: pointer;
  }

  .mask-content-history{
    width:400px;
    position: relative;
  }

  .mask-content-history .close-btn{
    position: absolute;
    top:10px;
    right:10px;
    cursor: pointer;
  }

  .mask-content-history .list{
    height:600px;
    overflow-y:scroll;
  }

  .mask-content-history .tap-box{
    display:flex;
    justify-content:center;
  }
  .mask-content-history .tap-box>div{
    margin:0 20px;
    font-size:16px;
    font-weight:bold;
    position:relative;
    cursor:pointer;
    padding-bottom:10px;
  }
  .mask-content-history .tap-box .active:after{
    display:block;
    content:"";
    width:30px;
    height:3px;
    border-radius:3px;
    background-color:red;
    position:absolute;
    bottom:0;
    left:50%;
    transform:translateX(-50%);
  }

  .mask-content-history div{
    color:#333;
    font-size:14px;
  }

  .spin-ani {
    animation: spin 5s infinite normal linear;
  }

  @keyframes spin {
    0% {
      transform: rotate(0deg);
    }
    50% {
      transform: rotate(180deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }

  .ball-ani {
    animation: ball 1s infinite normal linear;
  }

  @keyframes ball {
    0% {
      transform: rotate(0deg);
    }
    50% {
      transform: rotate(180deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }

  .history-wrapper {
    width: 100%;
    border-radius: 20px;
    padding: 5px;
    display: flex;
    justify-content: center;
    border: 1px solid #fff;
    margin-top: 20px;
    align-items: center;
  }

  .history {
    color: #fff;
    font-weight: 500;
    font-size: 24px;
  }

  [disabled] button, button[disabled] {
    box-shadow: none;
    background-color: var(--light-gray);
    color: gray;
    cursor: not-allowed;
    transform: none;
  }

  [disabled] .loader-content {
    text-indent: -900em;
    width: 2em;
    position: relative;
  }
  [disabled] .loader-content:after {
    content: " ";
    display: block;
    width: 0.8em;
    height: 0.8em;
    border-radius: 50%;
    border: 2px solid #fff;
    border-color: var(--fg) transparent var(--fg) transparent;
    animation: loader 1.2s linear infinite;
    position: absolute;
    top: 0.45em;
    right: 0.5em;
  }

  @keyframes loader {
    0% { transform: rotate(0deg) }
    100% { transform: rotate(360deg) }
  }

  fieldset {
    border: none;
  }

</style>

<style >
  .menu{
    position:fixed;
    right:40px;
    bottom:40px;
  }
  .menu-item{
    width:120px;
    height:46px;
    border-radius:10px;
    background:#FFF;
    display: flex;
    justify-content: center;
    align-items: center;
    font-size:12px;
    font-weight:bold;
    margin-top:20px;
    cursor:pointer;
  }


  .mask{
    position:fixed;
    top:0;
    left:0;
    right:0;
    bottom:0;
    z-index:999;
    background:rgba(0,0,0,0.7);
    display:flex;
    justify-content:center;
    align-items: center;
  }
  
</style>
