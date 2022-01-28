import 'package:dashboard/controllers/global/cancleButton.dart';
import 'package:dashboard/screens/app_create/components/createAppForm.dart';
import 'package:flutter/material.dart';

void createAppDialog(BuildContext context) {
  var data = MediaQuery.of(context).size;

  showDialog(
    context: context,
    builder: (BuildContext context) {
      return AlertDialog(
          title: AnimatedContainer(
              duration: Duration(milliseconds: 500),
              width: 350,
              height: 350,
              child: Container(
                padding: EdgeInsets.all(20),
                child: Column(
                  children: [
                    SelectableText(
                      "Create application",
                      style: Theme.of(context).textTheme.headline5,
                    ),
                    SizedBox(height: 20),
                    CreateAppForm(),
                    CancleButton()
                  ],
                ),
              )));
    },
  );
}
