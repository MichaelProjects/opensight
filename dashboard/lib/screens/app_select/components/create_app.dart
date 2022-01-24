import 'package:coolicons/coolicons.dart';
import 'package:dashboard/screens/app_create/app_create.dart';
import 'package:flutter/material.dart';

class CreateApp extends StatefulWidget {
  CreateApp({Key? key}) : super(key: key);

  @override
  _CreateAppState createState() => _CreateAppState();
}

class _CreateAppState extends State<CreateApp> {
  @override
  Widget build(BuildContext context) {
    return Container(
        margin: const EdgeInsets.only(
          top: 20,
        ),
        height: 100,
        width: 200,
        padding: const EdgeInsets.only(top: 10, bottom: 5),
        decoration: BoxDecoration(
          boxShadow: [
            BoxShadow(
                spreadRadius: 0.8,
                blurRadius: 3,
                offset: Offset(0, 1),
                color: Colors.grey)
          ],
          color: Theme.of(context).primaryColor,
          borderRadius: BorderRadius.circular(5),
        ),
        child: InkWell(
            onTap: () {
              createAppDialog(context);
            },
            onHover: (value) {
              print(value);
            },
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.center,
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Icon(Coolicons.plus, color: Colors.blue),
                Text(
                  "App hinzufügen",
                  style: Theme.of(context).textTheme.button,
                )
              ],
            )));
  }
}
