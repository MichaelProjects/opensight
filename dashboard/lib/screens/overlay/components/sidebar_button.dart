import 'package:flutter/material.dart';

class SidebarButton extends StatefulWidget {
  final String label;
  final IconData icon;
  final Function onPressed;
  final bool deactivated;
  const SidebarButton({
    Key? key,
    required this.label,
    required this.icon,
    required this.onPressed,
    required this.deactivated,
  }) : super(key: key);

  @override
  _SidebarButtonState createState() => _SidebarButtonState();
}

class _SidebarButtonState extends State<SidebarButton> {
  bool hoverState = false;
  @override
  Widget build(BuildContext context) {
    return InkWell(
        onHover: (value) {
          setState(() {
            hoverState = value;
          });
        },
        onTap: () {
          if (!widget.deactivated) {
            widget.onPressed();
          }
        },
        child: Container(
            margin: const EdgeInsets.all(5),
            padding: const EdgeInsets.only(left: 15),
            decoration: BoxDecoration(
              color: widget.deactivated == true
                  ? Theme.of(context).primaryColor.withOpacity(0)
                  : hoverState == false
                      ? Theme.of(context).primaryColor.withOpacity(0)
                      : Colors.blue.withOpacity(0.4),
              borderRadius: BorderRadius.circular(5),
            ),
            width: 190,
            height: 40,
            child: Row(
              children: [
                Icon(
                  widget.icon,
                ),
                const SizedBox(width: 10),
                Text(widget.label, style: Theme.of(context).textTheme.headline6)
              ],
            )));
  }
}
