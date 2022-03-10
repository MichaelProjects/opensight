import 'package:coolicons/coolicons.dart';
import 'package:dashboard/controllers/timeline_controller.dart';
import 'package:dashboard/glob_components/p_button.dart';
import 'package:flutter/material.dart';
import 'package:intl/intl.dart';
import 'package:provider/provider.dart';

class Timeline extends StatefulWidget {
  const Timeline({Key? key}) : super(key: key);

  @override
  _TimelineState createState() => _TimelineState();
}

class _TimelineState extends State<Timeline> {
  @override
  Widget build(BuildContext context) {
    TimelineController controller = Provider.of<TimelineController>(context);
    TextEditingController startTimeInput = TextEditingController();
    TextEditingController endTimeInput = TextEditingController();
    Future<void> showPopUpMenu(
        Offset globalPosition, BuildContext context) async {
      var size = MediaQuery.of(context).size;
      double left = globalPosition.dx;
      double top = globalPosition.dy;
      await showMenu(
        context: context,
        position: RelativeRect.fromLTRB(left, top, size.width - left, 0),
        items: [
          PopupMenuItem(
            value: 0,
            child: Padding(
              padding: const EdgeInsets.only(left: 0, right: 40),
              child: Row(
                children: [Text("Today")],
              ),
            ),
          ),
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
            value: 2,
            child: Padding(
              padding: const EdgeInsets.only(left: 0, right: 40),
              child: Row(
                children: [Text("Last 30 Days")],
              ),
            ),
          ),
          PopupMenuItem(
            value: 3,
            child: Padding(
              padding: const EdgeInsets.only(left: 0, right: 40),
              child: Row(
                children: [Text("This month")],
              ),
            ),
          ),
          PopupMenuItem(
            value: 4,
            enabled: false,
            child: Padding(
              padding: const EdgeInsets.only(left: 0, right: 40),
              child: Column(
                children: [
                  TextField(
                    controller: startTimeInput,
                    decoration: InputDecoration(labelText: "Start Timestamp"),
                  )
                ],
              ),
            ),
          ),
          PopupMenuItem(
            value: 5,
            enabled: false,
            child: Padding(
              padding: const EdgeInsets.only(left: 0, right: 40),
              child: Column(
                children: [
                  TextField(
                    controller: endTimeInput,
                    decoration: InputDecoration(labelText: "End Timestamp"),
                  )
                ],
              ),
            ),
          ),
          PopupMenuItem(
            value: 6,
            child: Padding(
                padding: const EdgeInsets.only(left: 0, right: 40),
                child: Center(child: Text("Search"))),
          ),
        ],
        elevation: 8.0,
      ).then((value) {
        switch (value) {
          case 0:
            controller.today();
            break;
          case 1:
            controller.getLast7Day();
            break;
          case 2:
            controller.getLast30Days();
            break;
          case 3:
            controller.getCurrentMonth();
            break;
          case 6:
            if (startTimeInput.text.trim().isNotEmpty &&
                endTimeInput.text.trim().isNotEmpty) {
              controller.custom(
                  int.parse(startTimeInput.text), int.parse(endTimeInput.text));
            } else {
              ScaffoldMessenger.of(context).showSnackBar(SnackBar(
                  content: Text("Please enter start and end timestamp")));
            }
            break;
        }
      });
    }

    DateFormat formatter = DateFormat('dd.MM.yyyy');
    String startTimeString = formatter
        .format(DateTime.fromMillisecondsSinceEpoch(controller.startTime));
    String endTimeString = formatter
        .format(DateTime.fromMillisecondsSinceEpoch(controller.endTime));

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
                Text("$startTimeString - $endTimeString"),
              ],
            )));
  }
}
