import 'package:coolicons/coolicons.dart';
import 'package:dashboard/glob_components/p_button.dart';
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
        child: GestureDetector(
            onTapDown: (TapDownDetails details) {
              print("Clicked");
              showPopUpMenu(details.globalPosition, context);
            },
            child: Row(
              children: [
                const Icon(Coolicons.calendar),
                const SizedBox(width: 15),
                const Text("30.10.2021 - 30.11.2021")
              ],
            )));
  }
}

Future<void> showPopUpMenu(Offset globalPosition, BuildContext context) async {
  var size = MediaQuery.of(context).size;
  double left = globalPosition.dx;
  double top = globalPosition.dy;
  await showMenu(
    context: context,
    position: RelativeRect.fromLTRB(left, top, size.width - left, 0),
    items: [
      PopupMenuItem(
        value: 1,
        child: Padding(
          padding: const EdgeInsets.only(left: 0, right: 40),
          child: Row(
            children: [Text("Last 7 Days")],
          ),
        ),
      ),
      PopupMenuItem(
        value: 1,
        child: Padding(
          padding: const EdgeInsets.only(left: 0, right: 40),
          child: Row(
            children: [Text("Last 30 Days")],
          ),
        ),
      ),
      PopupMenuItem(
        value: 2,
        enabled: false,
        child: Padding(
          padding: const EdgeInsets.only(left: 0, right: 40),
          child: Column(
            children: [
              TextField(
                decoration: InputDecoration(labelText: "Start Timestamp"),
              )
            ],
          ),
        ),
      ),
      PopupMenuItem(
        value: 2,
        enabled: false,
        child: Padding(
          padding: const EdgeInsets.only(left: 0, right: 40),
          child: Column(
            children: [
              TextField(
                decoration: InputDecoration(labelText: "End Timestamp"),
              )
            ],
          ),
        ),
      ),
      PopupMenuItem(
        child: Padding(
            padding: const EdgeInsets.only(left: 0, right: 40),
            child: Center(child: Text("Search"))),
      ),
    ],
    elevation: 8.0,
  ).then((value) {
    print(value);
    if (value == 1) {
      //do your task here for menu 1
    }
    if (value == 2) {
      //do your task here for menu 2
    }
    if (value == 3) {
      //do your task here for menu 3
    }
  });
}
