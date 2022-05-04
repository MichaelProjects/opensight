import 'package:dashboard/controllers/create_app_controller.dart';
import 'package:dashboard/glob_components/p_button.dart';
import 'package:dashboard/screens/app_create/components/platform_btn.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

List<Color> platformStatesColor = [Colors.white, Colors.white, Colors.white];

/// The state of the [CreateAppScreen] is managed with the [CreateAppController],
/// to ensure a save application configuration.
class CreateAppForm extends StatefulWidget {
  CreateAppForm({Key? key}) : super(key: key);

  @override
  State<CreateAppForm> createState() => _CreateAppFormState();
}

class _CreateAppFormState extends State<CreateAppForm> {
  List<GlobalKey<FormState>> formKeys = [
    GlobalKey<FormState>(),
    GlobalKey<FormState>(),
  ];
  String appname = "";

  @override
  void initState() {
    super.initState();
  }

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
              labelText: "app name",
              hintText: "eg Cardano Tracker"),
          validator: (value) {
            return validateFormFieldAppName(value!);
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
            return validateFormFieldPackageName(value!);
          },
          onSaved: (value) {
            appCreateController.setPackageName(value!);
          },
        ));

    // Platform selection

    Widget platformSelector = Row(
      crossAxisAlignment: CrossAxisAlignment.center,
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        const SizedBox(height: 50),
        PlatformBtn("img/icons/android.png", platformStatesColor[0], () {
          appCreateController.setPlatform(Plattform.android);
          setState(() {
            platformStatesColor[0] = Colors.blue;
            platformStatesColor[1] = Theme.of(context).backgroundColor;
            platformStatesColor[2] = Theme.of(context).backgroundColor;
          });
        }),
        PlatformBtn("img/icons/web.png", platformStatesColor[1], () {
          appCreateController.setPlatform(Plattform.web);
          setState(() {
            platformStatesColor[0] = Theme.of(context).backgroundColor;
            platformStatesColor[1] = Colors.blue;
            platformStatesColor[2] = Theme.of(context).backgroundColor;
          });
        }),
        PlatformBtn("img/icons/ios.png", platformStatesColor[2], () {
          appCreateController.setPlatform(Plattform.ios);
          setState(() {
            platformStatesColor[0] = Theme.of(context).backgroundColor;
            platformStatesColor[1] = Theme.of(context).backgroundColor;
            platformStatesColor[2] = Colors.blue;
          });
        }),
      ],
    );

    switch (appCreateController.state) {
      case CreateAppState.none:
        return const Text("Initial");
      case CreateAppState.enterName:
        return Form(
            key: formKeys[0],
            child: Column(
              children: [
                SelectableText("What is the name of your application?",
                    style: Theme.of(context).textTheme.subtitle1),
                const SizedBox(height: 50),
                appNameField,
                const SizedBox(height: 20),
                PButton("Next", Colors.blue, () {
                  if (formKeys[0].currentState!.validate()) {
                    formKeys[0].currentState!.save();
                    appCreateController.enterPlattform();
                  }
                }),
              ],
            ));
      case CreateAppState.enterPlattform:
        return Column(
          children: [
            SelectableText("On which platform does your app operate?",
                style: Theme.of(context).textTheme.subtitle1),
            SizedBox(height: 20),
            platformSelector,
            const SizedBox(height: 50),
            PButton("Next", Colors.blue, () {
              platformStatesColor[0] = Theme.of(context).backgroundColor;
              platformStatesColor[1] = Theme.of(context).backgroundColor;
              platformStatesColor[2] = Theme.of(context).backgroundColor;
              appCreateController.enterPackageName();
            }),
          ],
        );
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
                  if (formKeys[1].currentState!.validate()) {
                    formKeys[1].currentState!.save();

                    appCreateController.createApplication();
                  }
                }),
              ],
            ));
      case CreateAppState.created:
        return Text("Application created",
            style: Theme.of(context).textTheme.subtitle1);
      default:
        return const Text("Error");
    }
  }
}

/// Validates the form fields in [CreateAppForm].
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
