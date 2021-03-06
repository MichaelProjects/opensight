import 'package:coolicons/coolicons.dart';
import 'package:dashboard/controllers/create_app_controller.dart';
import 'package:dashboard/screens/app_create/app_create.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class CreateApp extends StatefulWidget {
  CreateApp({Key? key}) : super(key: key);

  @override
  _CreateAppState createState() => _CreateAppState();
}

class _CreateAppState extends State<CreateApp> {
  @override
  Widget build(BuildContext context) {
    CreateAppController appCreateController =
        Provider.of<CreateAppController>(context);
    Color widgetColor = Theme.of(context).primaryColor;
    return Container(
        margin: const EdgeInsets.all(10),
        padding: const EdgeInsets.only(),
        height: 103,
        width: 200,
        decoration: BoxDecoration(
          color: widgetColor,
          borderRadius: BorderRadius.circular(5),
        ),
        child: InkWell(
            onTap: () {
              appCreateController.enterName();
              createAppDialog(context);
            },
            onHover: (value) {
              setState(() {
                if (value) {
                  widgetColor = Theme.of(context).backgroundColor;
                } else {
                  widgetColor = Theme.of(context).primaryColor;
                }
              });
            },
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.center,
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                const Icon(Coolicons.plus, color: Colors.blue),
                Text(
                  "App hinzufügen",
                  style: Theme.of(context).textTheme.button,
                )
              ],
            )));
  }
}
