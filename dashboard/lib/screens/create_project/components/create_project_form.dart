import 'package:dashboard/controllers/create_app_controller.dart';
import 'package:dashboard/glob_components/p_button.dart';
import 'package:dashboard/screens/app_create/components/platform_btn.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

/// The state of the [CreateAppScreen] is managed with the [CreateAppController],
/// to ensure a save application configuration.
class CreateProjectForm extends StatefulWidget {
  CreateProjectForm({Key? key}) : super(key: key);

  @override
  State<CreateProjectForm> createState() => _CreateProjectFormState();
}

class _CreateProjectFormState extends State<CreateProjectForm> {
  List<GlobalKey<FormState>> formKeys = [
    GlobalKey<FormState>(),
    GlobalKey<FormState>(),
  ];

  String appname = "";
  @override
  Widget build(BuildContext context) {
    CreateAppController appCreateController =
        Provider.of<CreateAppController>(context);
    // Formfield for application detail input
    Widget appNameField = SizedBox(
        width: 300,
        child: TextFormField(
          decoration: const InputDecoration(
              enabledBorder: OutlineInputBorder(
                borderSide: BorderSide(color: Colors.grey),
              ),
              focusedBorder: OutlineInputBorder(
                borderSide: BorderSide(color: Colors.blue),
              ),
              labelText: "Project name",
              hintText: "eg Cardano Tracker"),
          validator: (value) {
            validateFormFieldAppName(value!);
          },
          onSaved: (value) {
            appCreateController.setName(value!);
          },
        ));
    Widget appPackageNameField = SizedBox(
        width: 300,
        child: TextFormField(
          decoration: const InputDecoration(
              enabledBorder: OutlineInputBorder(
                borderSide: BorderSide(color: Colors.grey),
              ),
              focusedBorder: OutlineInputBorder(
                borderSide: BorderSide(color: Colors.blue),
              ),
              labelText: "Package Name",
              hintText: "eg io.yourcardano.app"),
          validator: (value) {
            validateFormFieldPackageName(value!);
          },
          onSaved: (value) {
            appCreateController.setPackageName(value!);
          },
        ));

    switch (appCreateController.state) {
      case CreateAppState.none:
        return const Text("Initial");
      case CreateAppState.enterName:
        return Form(
            key: formKeys[0],
            child: Column(
              children: [
                SelectableText("What should be the name of your project?",
                    style: Theme.of(context).textTheme.subtitle1),
                const SizedBox(height: 50),
                appNameField,
                const SizedBox(height: 20),
                PButton("Next", Colors.blue, () {
                  formKeys[0].currentState!.save();
                  appCreateController.enterPackageName();
                }),
              ],
            ));
      case CreateAppState.enterPackageName:
        return Form(
            key: formKeys[1],
            child: Column(
              children: [
                SelectableText("What package name does your app have?",
                    style: Theme.of(context).textTheme.subtitle1),
                const SizedBox(height: 50),
                appPackageNameField,
                const SizedBox(height: 50),
                PButton("Next", Colors.blue, () {
                  formKeys[1].currentState!.save();
                  appCreateController.createApplication();
                }),
              ],
            ));
      case CreateAppState.created:
        return Text("Project created",
            style: Theme.of(context).textTheme.subtitle1);
      default:
        return const Text("Error");
    }
  }
}

/// Validates the form fields in [CreateProjectForm].
validateFormFieldAppName(String value) {
  if (value.isEmpty) {
    return "Enter a valid name";
  }
  return null;
}

validateFormFieldPackageName(String value) {
  if (value.isEmpty) {
    return "Enter a valid name";
  }
  return null;
}
