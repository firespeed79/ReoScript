﻿///////////////////////////////////////////////////////////////////////////////
//
// ReoScript Core Library
// 
// HP: http://www.unvell.com/ReoScript
// 
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY
// KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR
// PURPOSE.
//
// License: GNU Lesser General Public License (LGPLv3)
//
// Email: lujing@unvell.com
//
// Copyright (C) unvell, 2012-2013. All Rights Reserved
//
///////////////////////////////////////////////////////////////////////////////

// console
if (this.console != null) {
	this.console.log = function(t) { __stdout__(t); }
}

// Math
if (this.Math != null) {
  this.Math.PI = 3.141592653589793;
  this.Math.E = 2.71828182845904;
  this.Math.LN2 = 0.6931471805599453;
  this.Math.LN10 = 2.302585092994046;
}
