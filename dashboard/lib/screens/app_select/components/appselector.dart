// ignore_for_file: prefer_const_constructors

import 'package:dashboard/controllers/app_controller.dart';
import 'package:dashboard/model/application.dart';
import 'package:dashboard/screens/app_select/components/create_app.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class AppSelector extends StatefulWidget {
  List<Application> apps;
  AppSelector(this.apps);
  @override
  _AppSelectorState createState() => _AppSelectorState();
}

class _AppSelectorState extends State<AppSelector> {
  @override
  Widget build(BuildContext context) {
    ApplicationProvider appController =
        Provider.of<ApplicationProvider>(context);
    var data = MediaQuery.of(context).size;
    return Container(
        padding: EdgeInsets.all(20),
        height: 400,
        width: 230,
        child: Column(
          children: [
            SelectableText("Your projects",
                style: Theme.of(context).textTheme.headline6),
            CreateApp(),
          ],
        ));
  }
}
