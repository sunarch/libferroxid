// -*- coding: utf-8, vim: expandtab:ts=4 -*-

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Format: AB1
// 26 * 26 * 10 = 6,760
// with 3 letters reserved:
// 23 * 26 * 10 = 5,980

// Earth equatorial circumference: 40,075 km
// Earth meridional circumference: 40,008 km

// Format: AB1-CD2
// 3 letters reserved in the first digit (X,Y,Z)
// 5,980 * 6,760 = 40,424,800

// Format: AB-12-CD
// 676 * 100 * 676 = 45,697,600
// with 3 letters reserved: 40,424,800

// Jupiter equatorial circumference: 439,264 km

// Format: AB1-CD2-EF3
// 3 letters reserved in the first digit
// 5,980 * 6,760 * 6,760 = 273,271,648,000

// Format: 12-AB-34-CD-56
// 100 * 676 * 100 * 676 * 100 = 456,976,000,000
// with 3 number reserved: 319,883,200,000
// with 2 number reserved: 365,580,800,000
// with 1 number reserved: 411,278,400,000

// ---------------------------------------------------------------------

// Format: AB12
// 26 * 26 * 10 * 10 = 67,600
// with 3 letters reserved:
// 23 * 26 * 10 * 10 = 59,800

// Format: AB12-CD34
// 3 letters reserved in the first digit (X,Y,Z)
// 59,800 * 59,800 = 3,576,040,000

// ---------------------------------------------------------------------

// Format: letters or numbers
// 26 + 10 = 36

// 36 ^ 1 =            36  * 26 =             936
// 36 ^ 2 =         2,916  * 26 =          75,816
// 36 ^ 3 =       104,976  * 26 =       2,729,376
// 36 ^ 4 =     3,779,136  * 26 =      98,257,536
// 36 ^ 5 =   136,048,896  * 26 =   3,537,271,296
// 36 ^ 6 = 4,897,760,256  * 26 = 127,341,766,656
