
CLASS 1 : SUBTRATE BASIC

Nội dung: 

  + Giới thiệu Substrate 
  
  + Kiến trúc của Substrate 
  
  + Live coding 
  
 I - Giới thiệu :
 
      Blockchain là một sổ cái phi tập trung, lưu trữ dũ liệu trên nhiều node khác nhau 
      
      Các thành phần cơ bản của một node :
        
          - Lưu trữ dữ liệu
          
          - Cơ chế đồng thuận 
          
          - Giao thức giao tiếp giữa các node 
          
          - ...
          
      Substrate là một framework mã nguồn mở để xây dựng một blockchain hoàn chỉnh không cần phải xây dựng lại từ đầu 
      
      Ưu điểm :
        
          - Open source 
          
          - Khả năng tùy biến cao : có thể xây dựng được mạng blockchain Layer 1 tùy chỉnh từ cơ chế đồng thuận, phí gas, blocktime, ...
          
          - Khả năng mở rộng cao, linh hoạt ... Có thể sử dụng các pallet có sẵn được xây dựng từ cộng đồng 
          
          - Forkless - Upgrage chain không cần fork 
      
 II - Thành phần kiến trúc của Subtrate :
 
      Chia làm 2 thành phần : 
      
      Runtime  
      
          - Là thành phần xây dựng lên các chức năng của chain 
          
          - FRAME 
          
              + Runtime modules : Gồm nhiều pallet , mỗi pallet thực hiện 1 chức năng riêng 
              
              + System module : Đại diện cho các kiểu dữ liệu , thông tin block , strorage, ... 
              
              + Support library : Thư viện hỗ trợ xây dựng các macro, type, traits 
              
      Outernode 
      
  
  
