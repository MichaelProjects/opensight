// ignore_for_file: prefer_const_constructors

import 'package:dashboard/controllers/app_controller.dart';
import 'package:dashboard/glob_components/app_details.dart';
import 'package:dashboard/screens/app_select/components/create_app_button.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:routemaster/routemaster.dart';

class AppSelector extends StatefulWidget {
  AppSelector();
  @override
  _AppSelectorState createState() => _AppSelectorState();
}

class _AppSelectorState extends State<AppSelector> {
  @override
  Widget build(BuildContext context) {
    ApplicationModel appController = Provider.of<ApplicationModel>(context);
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
            Builder(builder: (context) {
              if (appController.apps.isEmpty) {
                return Center(child: Text("No projects yet"));
              } else {
                return Expanded(
                  child: ListView.builder(
                    itemCount: appController.apps.length,
                    itemBuilder: (context, index) {
                      return InkWell(
                          onHover: (value) {},
                          onTap: () {
                            appController
                                .setCurrentApp(appController.apps[index]);
                            Routemaster.of(context).push(
                                '/dashboard/${appController.apps[index].appID}');
                          },
                          child: Appdetails(
                            app: appController.apps[index],
                          ));
                    },
                  ),
                );
              }
            })
          ],
        ));
  }
}
