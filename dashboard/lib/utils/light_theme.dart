import 'package:dashboard/utils/text_theme.dart';
import 'package:flutter/material.dart';

ThemeData buildLightThemeData(BuildContext context) {
  return ThemeData(
      fontFamily: "Camby",
      textTheme: buildTextTheme(context),
      primarySwatch: Colors.blue,
      primaryColor: Colors.white,
      backgroundColor: Colors.white);
}
