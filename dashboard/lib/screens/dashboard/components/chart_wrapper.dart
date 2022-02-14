import 'package:flutter/material.dart';

class ChartWrapper extends StatelessWidget {
  Widget child;
  String title;
  ChartWrapper({required this.child, required this.title});

  @override
  Widget build(BuildContext context) {
    return Card(
        child: Column(children: [
      Container(
        child: Center(child: Text(this.title)),
        padding: EdgeInsets.only(left: 8, right: 8, top: 8, bottom: 8),
      ),
      Expanded(
          child: Padding(
              padding: const EdgeInsets.only(right: 18, top: 18, bottom: 18),
              child: this.child))
    ]));
  }
}
