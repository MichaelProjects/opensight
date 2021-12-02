import 'package:flutter/material.dart';

class SidebarButton extends StatefulWidget {
  final String label;
  final IconData icon;
  final Function onPressed;
  SidebarButton(
      {required this.label, required this.icon, required this.onPressed});

  @override
  _SidebarButtonState createState() => _SidebarButtonState();
}

class _SidebarButtonState extends State<SidebarButton> {
  bool hoverState = false;
  Widget build(BuildContext context) {
    return InkWell(
        onHover: (value) {
          setState(() {
            hoverState = value;
          });
        },
        onTap: () {
          widget.onPressed();
        },
        child: Container(
            margin: EdgeInsets.all(5),
            padding: EdgeInsets.only(left: 15),
            decoration: BoxDecoration(
              color: hoverState == false
                  ? Theme.of(context).primaryColor.withOpacity(0)
                  : Colors.blue.withOpacity(0.4),
              borderRadius: BorderRadius.circular(5),
            ),
            width: 200,
            height: 50,
            child: Row(
              children: [
                Icon(widget.icon),
                SizedBox(width: 10),
                Text(widget.label, style: Theme.of(context).textTheme.headline6)
              ],
            )));
  }
}
