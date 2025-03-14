import 'dart:typed_data';

import 'package:csv/csv.dart';
import 'dart:convert';

class DataConverter {
  // List of Rows || Each List Represents One Row
  final List<List<String>> content;

  DataConverter({
    required this.content,
  });

  
  Uint8List get convert {
    String csvContent = const ListToCsvConverter().convert([
      ...content,
    ]);

    final encoder = utf8.encode('\uFEFF');
    final encodedData = utf8.encode(csvContent);
    final bytes = Uint8List.fromList([...encoder, ...encodedData]);

    return bytes;
  }
}
