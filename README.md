# split-bill-stellar


<img width="1698" height="267" alt="image" src="https://github.com/user-attachments/assets/fb06d789-549c-4bc7-bb1c-2cc0ccd21856" />

ÊN DỰ ÁN: UniSplit

VẤN ĐỀ (1 câu):  

Sinh viên sống chung/đi du học thường phải chia hóa đơn và gửi tiền hỗ trợ xuyên biên giới nhưng bị ngân hàng/trung gian tính phí cao và xử lý chậm, gây bất tiện và dễ “quên trả”.

GIẢI PHÁP (1 câu):  

UniSplit dùng Stellar (USDC/XLM) và Soroban để tự động chia phần tiền của từng thành viên, thu tiền vào smart contract và thanh toán ngay cho người ứng tiền/chủ nhà chỉ trong vài giây với phí gần như bằng 0.

TÍNH NĂNG STELLAR SỬ DỤNG:  

[X] Chuyển XLM/USDC    [ ] Token tùy chỉnh    [X] Soroban contract  

[X] DEX tích hợp        [ ] Trustline          [ ] Clawback/Tuân thủ

NGƯỜI DÙNG MỤC TIÊU:  

Sinh viên (đặc biệt nhóm ở trọ ghép, nhóm làm đồ án) và du học sinh/nhóm bạn ở nhiều quốc gia cần chia tiền + gửi tiền nhanh.

TÍNH NĂNG CỐT LÕI (MVP):  

Giao dịch “Group-Pay Split”: 1 người tạo bill (tổng tiền + danh sách thành viên + số tiền mỗi người), từng thành viên nạp đúng phần USDC vào contract, khi đủ tổng thì contract tự động chuyển toàn bộ cho ví người nhận (người ứng tiền/chủ nhà).

TẠI SAO STELLAR:  

Truyền thống: chuyển khoản/ngân hàng (đặc biệt xuyên biên giới) thường mất 1–3 ngày làm việc và tốn phí cố định khoảng $15–$30/giao dịch; trên nhiều chain phí gas có thể vài đô đến hàng chục đô và thời gian/biến động phí khó đoán. Với Stellar, giao dịch hoàn tất trong khoảng 2–5 giây và phí cực thấp (mức rất nhỏ so với 1 cent), phù hợp thanh toán hóa đơn nhóm nhanh và thường xuyên.
