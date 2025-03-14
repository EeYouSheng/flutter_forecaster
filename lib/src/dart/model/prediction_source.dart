import 'package:intl/intl.dart';

class PredictionSource {
  final DateTime date;
  final double value;

  const PredictionSource({
    required this.date,
    required this.value,
  });

  PredictionSource.build({DateTime? date, double? value})
      : date = date ?? DateTime.now(),
        value = value ?? 0;

  List<String> get csvData =>
      [DateFormat('yyyy-MM-dd').format(date), value.toStringAsFixed(2)];
}
