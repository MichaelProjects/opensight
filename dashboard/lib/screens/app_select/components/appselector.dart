// ignore_for_file: prefer_const_constructors

import 'package:dashboard/controllers/dashboard/sidebar/app_controller.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class AppSelector extends StatefulWidget {
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
        color: Colors.blue,
        child: Column(
          children: [
            SelectableText("Your projects",
                style: Theme.of(context).textTheme.headline6),
            FutureBuilder(
                future: appController.fetchApplications(),
                builder: (context, snap) {
                  if (snap.connectionState != ConnectionState.done) {
                    return Center(child: CircularProgressIndicator());
                  }
                  return Text(snap.data.toString());
                })
          ],
        ));
  }
}
