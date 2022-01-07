import 'package:coolicons/coolicons.dart';
import 'package:flutter/material.dart';

class Timeline extends StatefulWidget {
  const Timeline({Key? key}) : super(key: key);

  @override
  _TimelineState createState() => _TimelineState();
}

class _TimelineState extends State<Timeline> {
  @override
  Widget build(BuildContext context) {
    return Container(
        width: 235,
        margin: const EdgeInsets.only(left: 20),
        padding: const EdgeInsets.all(10),
        height: 60,
        child: Row(
          children: [
            const Icon(Coolicons.calendar),
            const SizedBox(width: 15),
            const Text("30.10.2021 - 30.11.2021")
          ],
        ));
  }
}
