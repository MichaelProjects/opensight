import 'package:flutter/material.dart';

import '../../controllers/global/cancleButton.dart';
import 'components/create_project_form.dart';

void createProjectDialog(BuildContext context) {
  var data = MediaQuery.of(context).size;

  showDialog(
    context: context,
    builder: (BuildContext context) {
      return AlertDialog(
          title: AnimatedContainer(
              duration: const Duration(milliseconds: 500),
              width: 350,
              height: 350,
              child: Container(
                padding: const EdgeInsets.all(20),
                child: Column(
                  children: [
                    SelectableText(
                      "Create new project",
                      style: Theme.of(context).textTheme.headline5,
                    ),
                    const SizedBox(height: 20),
                    CreateProjectForm(),
                    CancleButton()
                  ],
                ),
              )));
    },
  );
}
