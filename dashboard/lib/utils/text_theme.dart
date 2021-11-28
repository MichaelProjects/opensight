import 'package:flutter/material.dart';

TextTheme buildTextTheme(BuildContext context) {
  return TextTheme(
      headline6: TextStyle(
          fontSize: 18, fontWeight: FontWeight.bold, letterSpacing: 1.0),
      subtitle1:
          TextStyle(fontSize: 15, color: Colors.grey, letterSpacing: 0.5));
}
