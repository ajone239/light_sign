/******************************************************************************/
/* Name: Austin Jones (ajone239)                                              */
/* Project: light_sign                                                        */
/* File: led_matrix_test.ino                                                  */
/*                                                                            */
/* Purpose: The code is to recieve strings over serial. These strings are then*/
/* displayed on a led sign. The display is achieved with the NeoMatix Library.*/
/*                                                                            */
/******************************************************************************/

#include <Adafruit_GFX.h>
#include <Adafruit_NeoMatrix.h>
#include <Adafruit_NeoPixel.h>
#ifndef PSTR
 #define PSTR // Make Arduino Due happy
#endif

#define PIN 6

// MATRIX DECLARATION:
// Parameter 1 = width of NeoPixel matrix
// Parameter 2 = height of matrix
// Parameter 3 = pin number (most are valid)
// Parameter 4 = matrix layout flags, add together as needed:
//   NEO_MATRIX_TOP, NEO_MATRIX_BOTTOM, NEO_MATRIX_LEFT, NEO_MATRIX_RIGHT:
//     Position of the FIRST LED in the matrix; pick two, e.g.
//     NEO_MATRIX_TOP + NEO_MATRIX_LEFT for the top-left corner.
//   NEO_MATRIX_ROWS, NEO_MATRIX_COLUMNS: LEDs are arranged in horizontal
//     rows or in vertical columns, respectively; pick one or the other.
//   NEO_MATRIX_PROGRESSIVE, NEO_MATRIX_ZIGZAG: all rows/columns proceed
//     in the same order, or alternate lines reverse direction; pick one.
//   See example below for these values in action.
// Parameter 5 = pixel type flags, add together as needed:
//   NEO_KHZ800  800 KHz bitstream (most NeoPixel products w/WS2812 LEDs)
//   NEO_KHZ400  400 KHz (classic 'v1' (not v2) FLORA pixels, WS2811 drivers)
//   NEO_GRB     Pixels are wired for GRB bitstream (most NeoPixel products)
//   NEO_GRBW    Pixels are wired for GRBW bitstream (RGB+W NeoPixel products)
//   NEO_RGB     Pixels are wired for RGB bitstream (v1 FLORA pixels, not v2)

Adafruit_NeoMatrix matrix = Adafruit_NeoMatrix(32, 8, PIN,
  NEO_MATRIX_TOP     + NEO_MATRIX_LEFT +
  NEO_MATRIX_COLUMNS + NEO_MATRIX_ZIGZAG,
  NEO_GRB            + NEO_KHZ800);

const uint16_t colors[] = {
  matrix.Color(255, 0, 0), matrix.Color(0, 255, 0), matrix.Color(0, 0, 255) };

int x = matrix.width();
int pass = 0;
String msg = "Austin Jones"; // Init str
int word_width = -36;

void setup() {

  Serial.begin(9600); // opens serial port,
  matrix.begin();

  matrix.setTextWrap(false);
  matrix.setBrightness(40);
  matrix.setTextColor(colors[0]);
}

void loop() {

  // Only read if there is data
  if (Serial.available()) {
    String tmp_msg = Serial.readString();
    // check for valid string
    if (tmp_msg.length() != 0) {
      msg = tmp_msg;
      Serial.println(msg);
      // calculate the word width for the code to display all your text
      word_width = -((msg.length() - 4) * 6);
      x = matrix.width();
    }
  }

  // Render this round on the matrix
  matrix.fillScreen(0);
  matrix.setCursor(x, 0);
  matrix.print(msg.c_str());

  // Move the x position and check if done with one display
  if(--x < word_width) {
    // Reset poss
    x = matrix.width();
    // change colour
    if(++pass >= 3) pass = 0;
    matrix.setTextColor(colors[pass]);

  }

  // Display
  matrix.show();
  delay(100);
}
