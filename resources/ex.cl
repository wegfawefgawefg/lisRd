(defclass vec3 ()
    ((x
      :initarg :x
      :initform 0
      :accessor x)
     (y
      :initarg :y
      :initform 0
      :accessor y)
     (z
      :initarg :z
      :initform 0
      :accessor z)))

(defmethod print-object ((obj vec3) stream)
  (print-unreadable-object (obj stream :type t :identity t)
    (format stream "x: ~a y: ~a z: ~a"
      (x obj) (y obj) (z obj))))

(defun make-blank-image (height width)
  (let ((array (make-array (list height width))))
    (dotimes (y height)
      (dotimes (x width)
        (setf (aref array y x) (make-instance 'vec3))))
    array))

(write-ppm3 "test.ppm" (make-blank-image 3 3))
(defun write-ppm3
    (filename pixels)
  (with-open-file (stream filename
                          :direction :output
                          :if-does-not-exist :create
                          :if-exists :overwrite)
    ; (let ((height (length pixels))
    ;       (width (length (aref pixels 0))))
    (destructuring-bind (height width) (array-dimensions pixels)
      (progn
       (format stream "P3~%")
       (format stream "~a ~a~%" width height))
      (format stream "255~%")
      (dotimes (y height)
        (dotimes (x width)
          (format stream "~a ~a ~a~%"
            (x (aref pixels y x))
            (y (aref pixels y x))
            (z (aref pixels y x))))))))


(pprint (make-blank-image 3 3))
(array-dimensions (make-blank-image 3 3))
(destructuring-bind (height width) (array-dimensions (make-blank-image 3 3))
  (format t "~a ~a" height width))

(defun draw-rect (pixels x y width height color)
  (destructuring-bind (pwidth pheight)
      (array-dimensions pixels)
    (let ((x1 (max 0 x))
          (y1 (max 0 y))
          (x2 (max 0 (min pwidth (+ x width))))
          (y2 (max 0 (min pheight (+ y height)))))
      (dotimes (y y1 y2)
        (dotimes (x x1 x2)
          (setf (aref pixels y x)
            (make-instance 'vec3
              :x (x color)
              :y (y color)
              :z (z color))))))))

(write-ppm3 "some_boxes.ppm"
            (let ((pixels (make-blank-image 100 100)))
              ;   (draw-rect pixels 10 10 20 20 (make-instance 'vec3 :x 255))
              (draw-rect pixels 80 80 20 20 (make-instance 'vec3 :y 255))
              ;   (draw-rect pixels 10 10 1 1 (make-instance 'vec3 :y 255))
              ;   (draw-rect pixels 30 30 20 20 (make-instance 'vec3 :y 255))
              ;   (draw-rect pixels 50 50 20 20 (make-instance 'vec3 :z 255))
              pixels))
