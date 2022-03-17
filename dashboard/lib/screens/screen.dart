import 'package:flutter/material.dart';
import 'package:dashboard/screens/overlay/overlay.dart' as topOverlay;

class MainScreen extends StatefulWidget {
  Widget child;
  MainScreen(this.child);

  @override
  State<MainScreen> createState() => MainScreenState();
}

class MainScreenState extends State<MainScreen> {
  @override
  Widget build(BuildContext context) {
    return topOverlay.Overlay(child: widget.child);
  }
}
