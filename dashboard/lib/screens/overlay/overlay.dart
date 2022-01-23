import 'package:dashboard/screens/overlay/sidebar.dart';
import 'package:flutter/material.dart';

import 'topbar.dart';

class Overlay extends StatefulWidget {
  final Widget child;
  Overlay({required this.child});

  @override
  OverlayState createState() => OverlayState();
}

class OverlayState extends State<Overlay> {
  @override
  Widget build(BuildContext context) {
    return Column(
        mainAxisAlignment: MainAxisAlignment.start,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const TopBar(),
          Row(
            children: [const Sidebar(), widget.child],
          )
        ]);
  }
}
