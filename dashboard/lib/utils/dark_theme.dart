import 'package:dashboard/utils/text_theme.dart';
import 'package:flutter/material.dart';

ThemeData buildDarkThemeData(BuildContext context) {
  return ThemeData(
      fontFamily: "Camby",
      textTheme: buildTextTheme(context),
      brightness: Brightness.dark,
      primaryColor: Colors.black38,
      backgroundColor: Colors.white);
}
